import os
import sys

WIDTH = 18
HEIGHT = 24

def _next(i):
	return next(i).rstrip()


name = os.path.basename(sys.argv[1]).rsplit('.', 1)[0].upper()

with open(sys.argv[1]) as fh:
	lines = iter(fh)
	assert _next(lines) == 'P3'
	assert _next(lines).startswith('#')
	assert _next(lines) == f'{WIDTH} {HEIGHT}'
	assert _next(lines) == '255'
	print(f'pub const {name}: Sprite = [')
	first = True
	for y in range(HEIGHT):
		if first:
			print('    [')
			first = False
		else:
			print('    ], [')
		for x in range(WIDTH):
			r = int(_next(lines), 10)
			g = int(_next(lines), 10)
			b = int(_next(lines), 10)
			print(f'        [{r}, {g}, {b}],')
	print('    ]')
	print('];')
	print()
