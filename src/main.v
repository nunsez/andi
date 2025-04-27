module main

import andi
import os

fn main() {
	mut files := os.ls('.') or { panic(err) }
	files.sort()
	files.reverse_in_place()
	andi.handle_anime(files) or { println('Skip anime diff because of error: ${err}') }
	andi.handle_manga(files) or { println('Skip manga diff because of error: ${err}') }
	os.input('Press enter to exit.')
}
