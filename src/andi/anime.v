module andi

import arrays
import encoding.xml

struct Anime {
pub:
	id               int
	title            string
	status           string
	score            int
	episodes_watched int
	source           string
}

fn Anime.make_shiki(node xml.XMLNode) !Anime {
	return Anime{
		id:               safe_int(value(node, 'series_animedb_id') or { '' })
		title:            value(node, 'series_title') or { '' }
		status:           (value(node, 'my_status') or { '' }).to_lower()
		score:            safe_int(value(node, 'my_score') or { '' })
		episodes_watched: safe_int(value(node, 'my_watched_episodes') or { '' })
		source:           'shiki'
	}
}

fn Anime.make_mal(node xml.XMLNode) !Anime {
	return Anime{
		id:               safe_int(value(node, 'series_animedb_id') or { '' })
		title:            value(node, 'series_title') or { '' }
		status:           (value(node, 'my_status') or { '' }).to_lower()
		score:            safe_int(value(node, 'my_score') or { '' })
		episodes_watched: safe_int(value(node, 'my_watched_episodes') or { '' })
		source:           'shiki'
	}
}

fn Anime.print_diff(diff []Anime) {
	if diff.len == 0 {
		println('No Anime diff!')
		return
	}

	println('Anime diff:')
	for anime in diff {
		println('${anime.id}: ${anime.title}')
	}
}

fn (a Anime) equal(o Anime) bool {
	if a.status == '' {
		return false
	}

	return a.id == o.id && a.status == o.status && a.score == o.score
		&& a.episodes_watched == o.episodes_watched
}

pub fn handle_anime(files []string) ! {
	shiki := get_shiki_animes(files)!
	mal := get_mal_animes(files)!
	diff := get_diff_anime(mal, shiki)
	Anime.print_diff(diff)
}

fn get_shiki_animes(files []string) ![]Anime {
	file := arrays.find_first(files, fn (file string) bool {
		return file.ends_with('_animes.xml')
	}) or { return error('shiki anime file not found') }

	doc := parse_shiki_doc(file)!

	nodes := doc.get_elements_by_tag('manga')
	mut result := []Anime{}
	for node in nodes {
		manga := Anime.make_shiki(node)!
		result << manga
	}
	return result
}

fn get_mal_animes(files []string) ![]Anime {
	file := arrays.find_first(files, fn (file string) bool {
		return file.starts_with('animelist_')
	}) or { return error('mal anime file not found') }

	doc := parse_mal_doc(file)!
	nodes := doc.get_elements_by_tag('anime')
	mut result := []Anime{}
	for node in nodes {
		result << Anime.make_mal(node)!
	}
	return result
}

fn get_diff_anime(list1 []Anime, list2 []Anime) []Anime {
	mut list1_map := map[int]Anime{}
	for item in list1 {
		list1_map[item.id] = item
	}

	mut list2_map := map[int]Anime{}
	for item in list2 {
		list2_map[item.id] = item
	}

	manga_eq := fn (a Anime, b ?Anime) bool {
		if b != none {
			return a.equal(b)
		}
		return false
	}

	diff1 := compare[Anime](list1_map, list2_map, manga_eq)
	diff2 := compare[Anime](list2_map, list1_map, manga_eq)

	anime_uniq := fn (a Anime, b ?Anime) bool {
		return b == none
	}
	uniq := compare[Anime](diff1, diff2, anime_uniq)

	return uniq.values()
}
