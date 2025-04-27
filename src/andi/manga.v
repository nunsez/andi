module andi

import arrays
import encoding.xml

struct Manga {
pub:
	id            int
	title         string
	status        string
	score         int
	chapters_read int
	volumes_read  int
	source        string
}

fn Manga.make_shiki(node xml.XMLNode) !Manga {
	return Manga{
		id:            safe_int(value(node, 'manga_mangadb_id') or { '' })
		title:         value(node, 'series_title') or { '' }
		status:        (value(node, 'my_status') or { '' }).to_lower()
		score:         safe_int(value(node, 'my_score') or { '' })
		chapters_read: safe_int(value(node, 'my_read_chapters') or { '' })
		volumes_read:  safe_int(value(node, 'my_read_volumes') or { '' })
		source:        'shiki'
	}
}

fn Manga.make_mal(node xml.XMLNode) !Manga {
	return Manga{
		id:            safe_int(value(node, 'manga_mangadb_id') or { '' })
		title:         value(node, 'manga_title') or { '' }
		status:        (value(node, 'my_status') or { '' }).to_lower()
		score:         safe_int(value(node, 'my_score') or { '' })
		chapters_read: safe_int(value(node, 'my_read_chapters') or { '' })
		volumes_read:  safe_int(value(node, 'my_read_volumes') or { '' })
		source:        'mal'
	}
}

fn Manga.print_diff(diff []Manga) {
	if diff.len == 0 {
		println('No Manga diff!')
		return
	}

	println('Manga diff:')
	for manga in diff {
		println('${manga.id}: ${manga.title}')
	}
}

fn (m Manga) equal(o Manga) bool {
	if m.status == '' {
		return false
	}

	if m.status == 'completed' {
		return m.id == o.id && m.status == o.status && m.score == o.score
			&& m.volumes_read == o.volumes_read
	}

	return m.id == o.id && m.status == o.status && m.score == o.score
		&& m.volumes_read == o.volumes_read && m.chapters_read == o.chapters_read
}

pub fn handle_manga(files []string) ! {
	shiki := get_shiki_mangas(files)!
	mal := get_mal_mangas(files)!
	diff := get_diff_manga(mal, shiki)
	Manga.print_diff(diff)
}

fn get_shiki_mangas(files []string) ![]Manga {
	file := arrays.find_first(files, fn (file string) bool {
		return file.ends_with('_mangas.xml')
	}) or { return error('shiki manga file not found') }

	doc := parse_shiki_doc(file)!
	nodes := doc.get_elements_by_tag('manga')
	mut result := []Manga{}
	for node in nodes {
		result << Manga.make_shiki(node)!
	}
	return result
}

fn get_mal_mangas(files []string) ![]Manga {
	file := arrays.find_first(files, fn (file string) bool {
		return file.starts_with('mangalist_')
	}) or { return error('mal manga file not found') }

	doc := parse_mal_doc(file)!
	nodes := doc.get_elements_by_tag('manga')
	mut result := []Manga{}
	for node in nodes {
		manga := Manga.make_mal(node)!
		result << manga
	}
	return result
}

fn get_diff_manga(list1 []Manga, list2 []Manga) []Manga {
	mut list1_map := map[int]Manga{}
	for item in list1 {
		list1_map[item.id] = item
	}

	mut list2_map := map[int]Manga{}
	for item in list2 {
		list2_map[item.id] = item
	}

	manga_eq := fn (a Manga, b ?Manga) bool {
		if b != none {
			return a.equal(b)
		}
		return false
	}

	diff1 := compare[Manga](list1_map, list2_map, manga_eq)
	diff2 := compare[Manga](list2_map, list1_map, manga_eq)

	manga_uniq := fn (a Manga, b ?Manga) bool {
		return b == none
	}
	uniq := compare[Manga](diff1, diff2, manga_uniq)

	return uniq.values()
}
