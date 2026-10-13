//! A small, bounded XML tree on top of `quick-xml`, plus a string-building XML writer.
//!
//! Element and attribute names are rewritten to *canonical prefixes* by namespace URI (`w:p`,
//! `r:id`, `a:blip`…), so the reader doesn't care which prefixes a producer chose. Names in
//! unknown namespaces keep explicit local declarations, so opaque extension markup survives
//! re-emission without being mistaken for a supported vocabulary.

use quick_xml::XmlVersion;
use quick_xml::events::Event;
use quick_xml::name::{QName, ResolveResult};
use quick_xml::reader::NsReader;
use std::sync::Arc;

use crate::DocxError;

/// Maximum element nesting accepted by the parser.
pub const MAX_DEPTH: usize = 256;
/// Maximum number of elements in one part.
pub const MAX_ELEMENTS: usize = 8_000_000;
/// Bound amplification when a long inherited namespace URI must be stored on many subtrees.
const MAX_NAMESPACE_URI: usize = 4096;
const MAX_PRESERVED_NAMESPACES: usize = 8 * 1024 * 1024;

/// Known namespaces and the canonical prefix we use for them (read and write).
pub const NAMESPACES: &[(&str, &str)] = &[
    ("w", "http://schemas.openxmlformats.org/wordprocessingml/2006/main"),
    ("r", "http://schemas.openxmlformats.org/officeDocument/2006/relationships"),
    ("wp", "http://schemas.openxmlformats.org/drawingml/2006/wordprocessingDrawing"),
    ("a", "http://schemas.openxmlformats.org/drawingml/2006/main"),
    ("c", "http://schemas.openxmlformats.org/drawingml/2006/chart"),
    ("pic", "http://schemas.openxmlformats.org/drawingml/2006/picture"),
    ("mc", "http://schemas.openxmlformats.org/markup-compatibility/2006"),
    ("w14", "http://schemas.microsoft.com/office/word/2010/wordml"),
    ("wpi", "http://schemas.microsoft.com/office/word/2010/wordprocessingInk"),
    ("w15", "http://schemas.microsoft.com/office/word/2012/wordml"),
    ("wp14", "http://schemas.microsoft.com/office/word/2010/wordprocessingDrawing"),
    ("wps", "http://schemas.microsoft.com/office/word/2010/wordprocessingShape"),
    ("wpg", "http://schemas.microsoft.com/office/word/2010/wordprocessingGroup"),
    ("wpc", "http://schemas.microsoft.com/office/word/2010/wordprocessingCanvas"),
    ("dgm", "http://schemas.openxmlformats.org/drawingml/2006/diagram"),
    ("dsp", "http://schemas.microsoft.com/office/drawing/2008/diagram"),
    ("m", "http://schemas.openxmlformats.org/officeDocument/2006/math"),
    ("v", "urn:schemas-microsoft-com:vml"),
    ("o", "urn:schemas-microsoft-com:office:office"),
    ("w10", "urn:schemas-microsoft-com:office:word"),
];

/// The `a:ext` URI under which a drawing is marked decorative ([MS-ODRAWXML]), and the
/// namespace of its `adec:decorative` element.
pub const DECORATIVE_EXT: &str = "{C183D7F6-B498-43B3-948B-1728B52AA6E4}";
pub const DECORATIVE_NS: &str = "http://schemas.microsoft.com/office/drawing/2017/decorative";

/// Namespaces that only appear in non-body parts.
const OTHER_NAMESPACES: &[(&str, &str)] = &[
    ("b", "http://schemas.openxmlformats.org/officeDocument/2006/bibliography"),
    ("rel", "http://schemas.openxmlformats.org/package/2006/relationships"),
    ("ct", "http://schemas.openxmlformats.org/package/2006/content-types"),
    ("cp", "http://schemas.openxmlformats.org/package/2006/metadata/core-properties"),
    ("dc", "http://purl.org/dc/elements/1.1/"),
    ("dcterms", "http://purl.org/dc/terms/"),
    ("xsi", "http://www.w3.org/2001/XMLSchema-instance"),
    ("xml", "http://www.w3.org/XML/1998/namespace"),
    ("ep", "http://schemas.openxmlformats.org/officeDocument/2006/extended-properties"),
    ("op", "http://schemas.openxmlformats.org/officeDocument/2006/custom-properties"),
    ("vt", "http://schemas.openxmlformats.org/officeDocument/2006/docPropsVTypes"),
];

/// ISO/IEC 29500 Strict namespaces map onto the transitional prefixes.
const STRICT: &[(&str, &str)] = &[
    ("w", "http://purl.oclc.org/ooxml/wordprocessingml/main"),
    ("r", "http://purl.oclc.org/ooxml/officeDocument/relationships"),
    ("wp", "http://purl.oclc.org/ooxml/drawingml/wordprocessingDrawing"),
    ("a", "http://purl.oclc.org/ooxml/drawingml/main"),
    ("pic", "http://purl.oclc.org/ooxml/drawingml/picture"),
    ("m", "http://purl.oclc.org/ooxml/officeDocument/math"),
];

fn prefix_for(uri: &[u8], extra: &[(&'static str, &str)]) -> &'static str {
    if let Some((p, _)) = extra.iter().find(|(_, u)| u.as_bytes() == uri) {
        return p;
    }
    for (p, u) in NAMESPACES.iter().chain(OTHER_NAMESPACES).chain(STRICT) {
        if u.as_bytes() == uri {
            return p;
        }
    }
    "?"
}

/// Canonicalize known namespaces; give unknown ones a locally declared prefix. Each element
/// carries the declarations it needs, including when extracted from its original parent.
fn qualified_name(uri: &[u8], local: &str, extra: &[(&'static str, &str)], declarations: &mut Vec<(String, String)>) -> Result<String, DocxError> {
    let known = prefix_for(uri, extra);
    if known != "?" {
        return Ok(format!("{known}:{local}"));
    }
    if uri.len() > MAX_NAMESPACE_URI {
        return Err(DocxError::Limit("namespace URI is too long to preserve".into()));
    }
    if let Some((prefix, _)) = declarations.iter().find(|(_, namespace)| namespace.as_bytes() == uri) {
        return Ok(format!("{prefix}:{local}"));
    }
    let prefix = format!("nw{}", declarations.len());
    let name = format!("{prefix}:{local}");
    declarations.push((prefix, String::from_utf8_lossy(uri).into_owned()));
    Ok(name)
}

/// Compatibility feature namespaces whose elements are not otherwise handled by this tree.
/// Keep the reader's existing feature set when canonicalizing `mc:Choice/@Requires` aliases.
fn requirement_prefix(uri: &[u8]) -> &'static str {
    let prefix = prefix_for(uri, &[]);
    if prefix != "?" {
        return prefix;
    }
    match uri {
        b"http://schemas.microsoft.com/office/drawing/2010/main" => "a14",
        b"http://schemas.microsoft.com/office/word/2015/wordml/symex" => "w16se",
        b"http://schemas.microsoft.com/office/word/2016/wordml/cid" => "w16cid",
        b"http://schemas.microsoft.com/office/word/2018/wordml" => "w16",
        b"http://schemas.microsoft.com/office/word/2018/wordml/cex" => "w16cex",
        b"http://schemas.microsoft.com/office/word/2020/wordml/sdtdatahash" => "w16sdtdh",
        _ => "?",
    }
}

const MC_NS: &str = "http://schemas.openxmlformats.org/markup-compatibility/2006";
/// MC lists are normally a handful of features. Bound both local lists and inherited policies.
const MAX_MC_NAMES: usize = 128;

/// Rewrite MC namespace-prefix/QName lists while the source bindings are in scope. Unknown
/// requirements retain a declared prefix, which remains outside the reader's understood set.
fn mce_list(
    value: &str,
    reader: &NsReader<&[u8]>,
    extra: &[(&'static str, &str)],
    declarations: &mut Vec<(String, String)>,
    ignorable: &mut Vec<Arc<str>>,
    attribute: &str,
) -> Result<String, DocxError> {
    let tokens: Vec<_> = value.split_whitespace().take(MAX_MC_NAMES + 1).collect();
    if tokens.len() > MAX_MC_NAMES || tokens.iter().any(|p| p.len() > 1024) {
        return Err(DocxError::Limit("too many or oversized markup compatibility names".into()));
    }
    let qnames = matches!(attribute, "mc:PreserveElements" | "mc:PreserveAttributes" | "mc:ProcessContent");
    let mut output = Vec::with_capacity(tokens.len());
    for token in tokens {
        let (prefix, local) = if qnames {
            let Some(pair) = token.split_once(':') else {
                output.push(token.to_string());
                continue;
            };
            pair
        } else {
            (token, "requirement")
        };
        let qualified = format!("{prefix}:{local}");
        let ResolveResult::Bound(namespace) = reader.resolver().resolve_attribute(QName(qualified.as_bytes())).0 else {
            // Preserve the reader's fallback for a malformed, undeclared requirement.
            output.push("?".into());
            continue;
        };
        let uri = namespace.as_ref();
        if uri.len() > MAX_NAMESPACE_URI {
            return Err(DocxError::Limit("namespace URI is too long to preserve".into()));
        }
        if attribute == "mc:Ignorable" && prefix_for(uri, extra) == "?" && !ignorable.iter().any(|n| n.as_bytes() == uri) {
            if ignorable.len() >= MAX_MC_NAMES {
                return Err(DocxError::Limit("too many inherited ignorable namespaces".into()));
            }
            ignorable.push(Arc::from(String::from_utf8_lossy(uri).as_ref()));
        }
        let name = if attribute == "Requires" && requirement_prefix(uri) != "?" {
            let canonical = requirement_prefix(uri);
            if prefix_for(uri, extra) == "?" && !declarations.iter().any(|(p, _)| p == canonical) {
                declarations.push((canonical.to_string(), String::from_utf8_lossy(uri).into_owned()));
            }
            format!("{canonical}:{local}")
        } else {
            qualified_name(uri, local, extra, declarations)?
        };
        output.push(if qnames { name } else { name.rsplit_once(':').map_or(name.clone(), |(p, _)| p.to_string()) });
    }
    Ok(output.join(" "))
}

/// An extracted extension subtree must retain its source's inherited ignorable policy. Apply
/// it to the locally declared foreign namespaces, rather than adding a blanket ignore policy.
fn localize_ignorable(el: &mut El, declarations: &mut Vec<(String, String)>, policy: &[Arc<str>]) {
    let prefixes: Vec<_> = declarations.iter().filter(|(_, uri)| policy.iter().any(|n| n.as_ref() == uri)).map(|(p, _)| p.clone()).collect();
    if prefixes.is_empty() {
        return;
    }
    if let Some((_, value)) = el.attrs.iter_mut().find(|(name, _)| name == "mc:Ignorable") {
        for prefix in prefixes {
            if !value.split_whitespace().any(|p| p == prefix) {
                if !value.is_empty() {
                    value.push(' ');
                }
                value.push_str(&prefix);
            }
        }
    } else {
        el.attrs.push(("mc:Ignorable".into(), prefixes.join(" ")));
    }
    // Kept fragments can also be emitted or parsed outside a document's body namespaces.
    declarations.push(("mc".into(), MC_NS.into()));
}

#[derive(Clone, Debug, PartialEq)]
pub enum Node {
    El(El),
    Text(String),
}

/// An element with canonical-prefixed name and attributes.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct El {
    pub name: String,
    pub attrs: Vec<(String, String)>,
    pub kids: Vec<Node>,
}

impl El {
    /// Attribute by canonical name (`w:val`); falls back to an unqualified attribute with the
    /// same local name (some producers omit the prefix).
    pub fn attr(&self, name: &str) -> Option<&str> {
        if let Some((_, v)) = self.attrs.iter().find(|(k, _)| k == name) {
            return Some(v);
        }
        let local = name.rsplit(':').next().unwrap_or(name);
        if local != name {
            return self.attrs.iter().find(|(k, _)| k == local).map(|(_, v)| v.as_str());
        }
        None
    }
    /// Child elements.
    pub fn els(&self) -> impl Iterator<Item = &El> {
        self.kids.iter().filter_map(|n| match n {
            Node::El(e) => Some(e),
            Node::Text(_) => None,
        })
    }
    pub fn child(&self, name: &str) -> Option<&El> {
        self.els().find(|e| e.name == name)
    }
    pub fn children<'a>(&'a self, name: &'a str) -> impl Iterator<Item = &'a El> + 'a {
        self.els().filter(move |e| e.name == name)
    }
    /// `w:val` of a child.
    pub fn child_val(&self, name: &str) -> Option<&str> {
        self.child(name).and_then(|c| c.attr("w:val"))
    }
    /// Direct text content.
    pub fn text(&self) -> String {
        let mut s = String::new();
        for n in &self.kids {
            if let Node::Text(t) = n {
                s.push_str(t);
            }
        }
        s
    }
    /// All descendant text (depth-bounded by the parser).
    pub fn deep_text(&self) -> String {
        let mut s = String::new();
        fn go(e: &El, s: &mut String, depth: usize) {
            if depth > MAX_DEPTH {
                return;
            }
            for n in &e.kids {
                match n {
                    Node::Text(t) => s.push_str(t),
                    Node::El(c) => go(c, s, depth + 1),
                }
            }
        }
        go(self, &mut s, 0);
        s
    }
    /// First descendant with this name (depth-first, bounded).
    pub fn find(&self, name: &str) -> Option<&El> {
        fn go<'a>(e: &'a El, name: &str, depth: usize) -> Option<&'a El> {
            if depth > MAX_DEPTH {
                return None;
            }
            for c in e.els() {
                if c.name == name {
                    return Some(c);
                }
                if let Some(f) = go(c, name, depth + 1) {
                    return Some(f);
                }
            }
            None
        }
        go(self, name, 0)
    }
    /// The element as XML with canonical prefixes (for re-emitting kept content). Unknown
    /// namespaces carry their own declarations; unsupported markup is kept without evaluation.
    pub fn to_xml(&self) -> String {
        fn go(e: &El, s: &mut String, depth: usize) {
            if depth > MAX_DEPTH || e.name.starts_with("?:") {
                return;
            }
            s.push('<');
            s.push_str(&e.name);
            for (k, v) in &e.attrs {
                if k.starts_with("?:") {
                    continue;
                }
                s.push(' ');
                s.push_str(k);
                s.push_str("=\"");
                s.push_str(&esc(v));
                s.push('"');
            }
            if e.kids.is_empty() {
                s.push_str("/>");
                return;
            }
            s.push('>');
            for n in &e.kids {
                match n {
                    Node::Text(t) => s.push_str(&esc(t)),
                    Node::El(c) => go(c, s, depth + 1),
                }
            }
            s.push_str("</");
            s.push_str(&e.name);
            s.push('>');
        }
        let mut s = String::new();
        go(self, &mut s, 0);
        s
    }
    /// Local part of the name.
    pub fn local(&self) -> &str {
        self.name.rsplit(':').next().unwrap_or(&self.name)
    }
}

/// UTF-16 LE/BE text with a byte-order mark as UTF-8 (`None` without one). Lossy: an unpaired
/// surrogate becomes U+FFFD and a trailing odd byte is dropped. The result is at most 1.5× the input.
pub(crate) fn utf16_to_utf8(bytes: &[u8]) -> Option<String> {
    let (rest, le) = match bytes.get(..2) {
        Some([0xFF, 0xFE]) => (bytes.get(2..)?, true),
        Some([0xFE, 0xFF]) => (bytes.get(2..)?, false),
        _ => return None,
    };
    let units = rest.as_chunks::<2>().0.iter().map(|c| if le { u16::from_le_bytes(*c) } else { u16::from_be_bytes(*c) });
    Some(char::decode_utf16(units).map(|c| c.unwrap_or(char::REPLACEMENT_CHARACTER)).collect())
}

/// Parse XML bytes into the root element.
pub fn parse(bytes: &[u8]) -> Result<El, DocxError> {
    parse_ns(bytes, &[])
}

/// [`parse`] with `extra` namespaces (prefix, URI) mapped to canonical prefixes first, so a
/// related vocabulary (WordprocessingML 2003) reads with the prefixes the reader knows.
pub fn parse_ns(bytes: &[u8], extra: &[(&'static str, &str)]) -> Result<El, DocxError> {
    // UTF-16 (with a BOM) is transcoded to UTF-8; a UTF-8 BOM is stripped.
    let transcoded = utf16_to_utf8(bytes);
    let bytes = match &transcoded {
        Some(s) => s.as_bytes(),
        None => bytes.strip_prefix(&[0xEF, 0xBB, 0xBF][..]).unwrap_or(bytes),
    };
    let mut r = NsReader::from_reader(bytes);
    {
        let c = r.config_mut();
        c.trim_text_start = false;
        c.trim_text_end = false;
        c.expand_empty_elements = false;
        c.check_end_names = false;
    }
    let mut stack: Vec<El> = Vec::new();
    let mut ignorable_stack: Vec<Vec<Arc<str>>> = Vec::new();
    let mut root: Option<El> = None;
    let mut count = 0usize;
    let mut namespace_bytes = 0usize;
    let mut buf = Vec::new();
    loop {
        buf.clear();
        let (res, ev) = r.read_resolved_event_into(&mut buf).map_err(|e| DocxError::Xml(e.to_string()))?;
        match ev {
            Event::Start(ref e) | Event::Empty(ref e) => {
                count += 1;
                if count > MAX_ELEMENTS {
                    return Err(DocxError::Limit("too many XML elements".into()));
                }
                let local = String::from_utf8_lossy(e.local_name().as_ref()).into_owned();
                let mut declarations = Vec::new();
                let name = match res {
                    ResolveResult::Bound(ns) => qualified_name(ns.as_ref(), &local, extra, &mut declarations)?,
                    _ => local,
                };
                let mut el = El { name, attrs: Vec::new(), kids: Vec::new() };
                let mut ignorable = ignorable_stack.last().cloned().unwrap_or_default();
                for a in e.attributes().with_checks(false) {
                    let Ok(a) = a else { continue };
                    let key = a.key;
                    let raw_key = key.as_ref();
                    if raw_key == b"xmlns" || raw_key.starts_with(b"xmlns:") {
                        continue;
                    }
                    let (ares, alocal) = r.resolver_mut().resolve_attribute(key);
                    let alocal = String::from_utf8_lossy(alocal.as_ref()).into_owned();
                    let aname = match ares {
                        ResolveResult::Bound(ns) => qualified_name(ns.as_ref(), &alocal, extra, &mut declarations)?,
                        _ => alocal,
                    };
                    let val = match a.normalized_value(XmlVersion::Explicit1_0) {
                        Ok(v) => v.into_owned(),
                        Err(_) => String::from_utf8_lossy(&a.value).into_owned(),
                    };
                    let val = if (el.name == "mc:Choice" && aname == "Requires")
                        || matches!(
                            aname.as_str(),
                            "mc:Ignorable" | "mc:MustUnderstand" | "mc:PreserveElements" | "mc:PreserveAttributes" | "mc:ProcessContent"
                        ) {
                        mce_list(&val, &r, extra, &mut declarations, &mut ignorable, &aname)?
                    } else {
                        val
                    };
                    el.attrs.push((aname, val));
                }
                localize_ignorable(&mut el, &mut declarations, &ignorable);
                namespace_bytes =
                    namespace_bytes.saturating_add(declarations.iter().map(|(prefix, namespace)| prefix.len() + namespace.len()).sum::<usize>());
                if namespace_bytes > MAX_PRESERVED_NAMESPACES {
                    return Err(DocxError::Limit("too much preserved namespace data".into()));
                }
                el.attrs.extend(declarations.into_iter().map(|(prefix, namespace)| (format!("xmlns:{prefix}"), namespace)));
                if matches!(ev, Event::Start(_)) {
                    if stack.len() >= MAX_DEPTH {
                        return Err(DocxError::Limit("XML nesting too deep".into()));
                    }
                    ignorable_stack.push(ignorable);
                    stack.push(el);
                } else {
                    attach(&mut stack, &mut root, el);
                }
            }
            Event::End(_) => {
                ignorable_stack.pop();
                if let Some(el) = stack.pop() {
                    attach(&mut stack, &mut root, el);
                }
            }
            Event::Text(t) => {
                if let Some(top) = stack.last_mut() {
                    let s = t.decode().map_err(|e| DocxError::Xml(e.to_string()))?;
                    push_text(top, &s);
                }
            }
            Event::CData(t) => {
                if let Some(top) = stack.last_mut() {
                    let s = String::from_utf8_lossy(&t).into_owned();
                    push_text(top, &s);
                }
            }
            Event::GeneralRef(g) => {
                if let Some(top) = stack.last_mut() {
                    let ch = match g.resolve_char_ref() {
                        Ok(Some(c)) => Some(c.to_string()),
                        Ok(None) => {
                            let n = g.decode().map_err(|e| DocxError::Xml(e.to_string()))?;
                            quick_xml::escape::resolve_predefined_entity(&n).map(str::to_string)
                        }
                        Err(_) => None,
                    };
                    if let Some(c) = ch {
                        push_text(top, &c);
                    }
                }
            }
            Event::Eof => break,
            _ => {}
        }
        if root.is_some() && stack.is_empty() {
            break;
        }
    }
    // Unclosed elements at EOF: close them (lenient).
    while let Some(el) = stack.pop() {
        attach(&mut stack, &mut root, el);
    }
    root.ok_or_else(|| DocxError::Xml("no root element".into()))
}

fn push_text(top: &mut El, s: &str) {
    if let Some(Node::Text(prev)) = top.kids.last_mut() {
        prev.push_str(s);
    } else {
        top.kids.push(Node::Text(s.to_string()));
    }
}

fn attach(stack: &mut [El], root: &mut Option<El>, el: El) {
    match stack.last_mut() {
        Some(parent) => parent.kids.push(Node::El(el)),
        None => {
            if root.is_none() {
                *root = Some(el);
            }
        }
    }
}

/// Is `c` allowed in XML 1.0 text?
fn xml_char_ok(c: char) -> bool {
    matches!(c, '\t' | '\n' | '\r' | '\u{20}'..='\u{D7FF}' | '\u{E000}'..='\u{FFFD}' | '\u{10000}'..='\u{10FFFF}')
}

/// Escape text for element content or attribute values (drops characters XML can't carry).
pub fn esc(s: &str) -> String {
    let mut o = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => o.push_str("&amp;"),
            '<' => o.push_str("&lt;"),
            '>' => o.push_str("&gt;"),
            '"' => o.push_str("&quot;"),
            '\'' => o.push_str("&apos;"),
            '\t' => o.push_str("&#9;"),
            '\n' => o.push_str("&#10;"),
            '\r' => o.push_str("&#13;"),
            c if xml_char_ok(c) => o.push(c),
            _ => {}
        }
    }
    o
}

/// A string-building XML writer.
#[derive(Default)]
pub struct W {
    pub s: String,
}

impl W {
    pub fn new() -> W {
        W { s: String::from("<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\n") }
    }
    fn attrs(&mut self, attrs: &[(&str, &str)]) {
        for (k, v) in attrs {
            self.s.push(' ');
            self.s.push_str(k);
            self.s.push_str("=\"");
            self.s.push_str(&esc(v));
            self.s.push('"');
        }
    }
    pub fn open(&mut self, name: &str, attrs: &[(&str, &str)]) {
        self.s.push('<');
        self.s.push_str(name);
        self.attrs(attrs);
        self.s.push('>');
    }
    pub fn empty(&mut self, name: &str, attrs: &[(&str, &str)]) {
        self.s.push('<');
        self.s.push_str(name);
        self.attrs(attrs);
        self.s.push_str("/>");
    }
    pub fn close(&mut self, name: &str) {
        self.s.push_str("</");
        self.s.push_str(name);
        self.s.push('>');
    }
    pub fn text(&mut self, t: &str) {
        self.s.push_str(&esc(t));
    }
    /// `<name>text</name>`.
    pub fn leaf(&mut self, name: &str, attrs: &[(&str, &str)], t: &str) {
        self.open(name, attrs);
        self.text(t);
        self.close(name);
    }
    /// `<name w:val="v"/>`.
    pub fn val(&mut self, name: &str, v: &str) {
        self.empty(name, &[("w:val", v)]);
    }
    pub fn raw(&mut self, s: &str) {
        self.s.push_str(s);
    }
    pub fn into_bytes(self) -> Vec<u8> {
        self.s.into_bytes()
    }
}

/// `xmlns:` declarations for the body-part namespaces, plus `mc:Ignorable`.
pub fn body_ns() -> Vec<(String, String)> {
    let mut v: Vec<(String, String)> = NAMESPACES.iter().map(|(p, u)| (format!("xmlns:{p}"), (*u).to_string())).collect();
    v.push(("mc:Ignorable".into(), "w14 w15 wp14".into()));
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn utf16_parts_parse_and_odd_ones_never_panic() {
        let src = r#"<?xml version="1.0" encoding="UTF-16"?><r:Relationships xmlns:r="urn:x"><r:Relationship Id="rId1" Target="café😀.xml"/></r:Relationships>"#;
        for le in [true, false] {
            let mut b = if le { vec![0xFF, 0xFE] } else { vec![0xFE, 0xFF] };
            b.extend(src.encode_utf16().flat_map(|u| if le { u.to_le_bytes() } else { u.to_be_bytes() }));
            let root = parse(&b).unwrap();
            assert_eq!(root.els().next().and_then(|e| e.attr("Target")), Some("café😀.xml"), "le {le}");
        }
        // Odd length, unpaired surrogates, a bare BOM: an error or a tree, never a panic.
        for b in
            [&[0xFF, 0xFE][..], &[0xFE, 0xFF, 0x00], &[0xFF, 0xFE, b'<', 0, 0x00, 0xD8, b'a', 0, b'/', 0, b'>', 0], &[0xFF, 0xFE, 0x00, 0xDC, 0x00]]
        {
            let _ = parse(b);
        }
        assert_eq!(utf16_to_utf8(&[0xFF, 0xFE, b'a', 0, 0x00, 0xD8, b'b']).as_deref(), Some("a\u{FFFD}"));
        assert_eq!(utf16_to_utf8(b"<a/>"), None);
    }

    #[test]
    fn parses_with_namespaces() {
        let x = br#"<?xml version="1.0"?><x:document xmlns:x="http://schemas.openxmlformats.org/wordprocessingml/2006/main" xmlns:rr="http://schemas.openxmlformats.org/officeDocument/2006/relationships"><x:p rr:id="r1" x:val="a&amp;b"><x:t xml:space="preserve"> hi &#65;&lt; </x:t></x:p></x:document>"#;
        let e = parse(x).unwrap();
        assert_eq!(e.name, "w:document");
        let p = e.child("w:p").unwrap();
        assert_eq!(p.attr("r:id"), Some("r1"));
        assert_eq!(p.attr("w:val"), Some("a&b"));
        let t = p.child("w:t").unwrap();
        assert_eq!(t.text(), " hi A< ");
        assert_eq!(t.attr("xml:space"), Some("preserve"));
    }

    #[test]
    fn choice_requirements_resolve_namespace_aliases_and_rebindings() {
        let x = br#"<mc:AlternateContent xmlns:mc="http://schemas.openxmlformats.org/markup-compatibility/2006" xmlns:pen="http://schemas.microsoft.com/office/word/2010/wordprocessingInk"><mc:Choice Requires="pen"/><mc:Choice xmlns:pen="urn:future" Requires="pen"/><mc:Choice Requires="undeclared"/></mc:AlternateContent>"#;
        let e = parse(x).unwrap();
        let values: Vec<_> = e.children("mc:Choice").map(|c| c.attr("Requires")).collect();
        assert_eq!(values, [Some("wpi"), Some("nw0"), Some("?")]);
        let foreign = e.children("mc:Choice").nth(1).unwrap();
        assert!(foreign.to_xml().contains("xmlns:nw0=\"urn:future\""));
    }

    #[test]
    fn inherited_ignorable_policy_survives_subtree_extraction_and_prefix_rebinding() {
        let root = parse(br#"<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" xmlns:mc="http://schemas.openxmlformats.org/markup-compatibility/2006" xmlns:v="urn:not-word:test:vendor" mc:Ignorable="v"><v:extension v:flag="keep"><v:child/><v:child xmlns:v="urn:not-word:test:required"/></v:extension></w:document>"#).unwrap();
        let extension = root.els().next().unwrap();
        assert_eq!(extension.attr("mc:Ignorable"), Some("nw0"));
        let children: Vec<_> = extension.els().collect();
        assert_eq!(children[0].attr("mc:Ignorable"), Some("nw0"));
        assert_eq!(children[1].attr("mc:Ignorable"), None, "a rebound URI does not inherit the old URI's policy");
        let xml = extension.to_xml();
        assert!(xml.contains("xmlns:mc=\"http://schemas.openxmlformats.org/markup-compatibility/2006\""));
        assert_eq!(parse(xml.as_bytes()).unwrap(), *extension);
    }

    #[test]
    fn compatibility_qname_lists_and_feature_requirements_use_declared_prefixes() {
        let root = parse(br#"<mc:Choice xmlns:mc="http://schemas.openxmlformats.org/markup-compatibility/2006" xmlns:v="urn:not-word:test:vendor" xmlns:feature="http://schemas.microsoft.com/office/drawing/2010/main" Requires="v feature" mc:Ignorable="v" mc:PreserveElements="v:item v:*" mc:PreserveAttributes="v:flag" mc:ProcessContent="v:wrapper"/>"#).unwrap();
        assert_eq!(root.attr("Requires"), Some("nw0 a14"));
        assert_eq!(root.attr("mc:PreserveElements"), Some("nw0:item nw0:*"));
        assert_eq!(root.attr("mc:PreserveAttributes"), Some("nw0:flag"));
        assert_eq!(root.attr("mc:ProcessContent"), Some("nw0:wrapper"));
        let xml = root.to_xml();
        assert!(xml.contains("xmlns:a14=\"http://schemas.microsoft.com/office/drawing/2010/main\""));
        assert_eq!(parse(xml.as_bytes()).unwrap(), root);
    }

    #[test]
    fn excessive_compatibility_lists_are_bounded() {
        let names = "v ".repeat(MAX_MC_NAMES + 1);
        let xml = format!("<a xmlns:mc=\"{MC_NS}\" xmlns:v=\"urn:vendor\" mc:Ignorable=\"{names}\"/>");
        assert!(matches!(parse(xml.as_bytes()), Err(DocxError::Limit(_))));
    }

    #[test]
    fn inherited_foreign_namespace_storage_is_bounded() {
        let too_long = format!("<v:root xmlns:v=\"urn:{}\"/>", "x".repeat(MAX_NAMESPACE_URI));
        assert!(matches!(parse(too_long.as_bytes()), Err(DocxError::Limit(_))));
        let namespace = format!("urn:{}", "x".repeat(MAX_NAMESPACE_URI - 4));
        let nodes = "<v:child/>".repeat(MAX_PRESERVED_NAMESPACES / MAX_NAMESPACE_URI + 1);
        let amplified = format!("<v:root xmlns:v=\"{namespace}\">{nodes}</v:root>");
        assert!(amplified.len() < 64 * 1024, "the source itself is small");
        assert!(matches!(parse(amplified.as_bytes()), Err(DocxError::Limit(_))));
    }

    #[test]
    fn unknown_namespaces_preserve_elements_attributes_and_rebindings() {
        let source = br#"<vendor:extension xmlns:vendor="urn:not-word:test:vendor" xmlns:other="urn:not-word:test:other" vendor:flag="keep" other:value="distinct"><vendor:data>opaque &amp; safe</vendor:data><vendor:data xmlns:vendor="urn:not-word:test:rebound" vendor:flag="second"/></vendor:extension>"#;
        let original = parse(source).unwrap();
        let serialized = original.to_xml();
        assert!(serialized.contains("urn:not-word:test:vendor"));
        assert!(serialized.contains("urn:not-word:test:other"));
        assert!(serialized.contains("urn:not-word:test:rebound"));
        assert!(serialized.contains("opaque &amp; safe"));
        assert_eq!(parse(serialized.as_bytes()).unwrap(), original);
        // A subtree also keeps the namespace inherited from the source's root.
        let child = original.els().next().unwrap();
        assert_eq!(parse(child.to_xml().as_bytes()).unwrap(), *child);
    }

    #[test]
    fn foreign_attributes_do_not_become_wordprocessingml_attributes() {
        let root = parse(br#"<w:color xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" xmlns:future="urn:not-word:future" w:val="112233" future:val="keep"/>"#).unwrap();
        assert_eq!(root.attr("w:val"), Some("112233"));
        assert_eq!(root.attr("nw0:val"), Some("keep"));
        assert!(root.to_xml().contains("urn:not-word:future"));
    }

    #[test]
    fn deep_nesting_is_error() {
        let mut s = String::new();
        for _ in 0..1000 {
            s.push_str("<a>");
        }
        assert!(parse(s.as_bytes()).is_err());
    }

    /// A start tag with thousands of `xmlns:` declarations is refused instead of allocated
    /// (RUSTSEC-2026-0195: unbounded namespace bindings in the resolver).
    #[test]
    fn too_many_namespace_declarations_is_error() {
        let mut s = String::from("<a xmlns:a0=\"urn:a0\"");
        for i in 1..10_000 {
            s.push_str(&format!(" xmlns:a{i}=\"urn:a{i}\""));
        }
        s.push_str("><b/></a>");
        assert!(parse(s.as_bytes()).is_err());
    }

    #[test]
    fn garbage_is_error_not_panic() {
        assert!(parse(b"").is_err());
        assert!(parse(b"\xff\xfe<").is_err());
        let _ = parse(b"<a><b></a>");
        let _ = parse(b"<a x='1' x='2'/>");
    }

    #[test]
    fn escapes() {
        assert_eq!(esc("a<b&\"\u{1}"), "a&lt;b&amp;&quot;");
    }
}
