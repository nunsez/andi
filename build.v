module main

import os

fn main() {
	sys := arguments()[1] or { os.user_os() }
	out := match sys {
		'linux' {
			'andi'
		}
		'windows' {
			'andi.exe'
		}
		else {
			eprintln('invalid os: ${sys}')
			return
		}
	}
	os.system('v ./src/main.v -os ${sys} -o ${out}')
}
