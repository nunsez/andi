module andi

import compress.gzip
import encoding.xml
import os

fn parse_mal_doc(filepath string) !xml.XMLDocument {
	compressed := os.read_bytes(filepath)!
	decompressed := gzip.decompress(compressed)!
	content := decompressed.bytestr()
	doc := xml.XMLDocument.from_string(content)!
	return doc
}

fn parse_shiki_doc(filepath string) !xml.XMLDocument {
	doc := xml.XMLDocument.from_file(filepath)!
	return doc
}

fn safe_int(value string) int {
	return int(value.parse_int(10, 0) or { 0 })
}

fn value(node xml.XMLNode, tag string) ?string {
	els := node.get_elements_by_tag(tag)
	el := els[0] or { return none }
	child := el.children[0] or { return none }
	return match child {
		xml.XMLNode { none }
		xml.XMLCData { child.text }
		xml.XMLComment { child.text }
		string { child }
	}
}

fn compare[T](map1 map[int]T, map2 map[int]T, eq fn (a T, b ?T) bool) map[int]T {
	mut result := map[int]T{}

	for key, value in map1 {
		mut is_equal := false

		if other := map2[key] {
			is_equal = eq(value, other)
		} else {
			is_equal = eq(value, ?T(none))
		}

		if !is_equal {
			result[key] = value
		}
	}

	return result
}
