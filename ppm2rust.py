import sys

WIDTH = 18
HEIGHT = 24

def _next(i):
	return next(i).rstrip()

with open(sys.argv[1]) as fh:
	lines = iter(fh)
	assert _next(lines) == 'P3'
	assert _next(lines).startswith('#')
	assert _next(lines) == f'{WIDTH} {HEIGHT}'
	assert _next(lines) == '255'
	print(f'pub const SPRITE: Sprite = [')
	print('    [')
	for y in range(HEIGHT):
		for x in range(WIDTH):
			r = int(_next(lines), 10)
			g = int(_next(lines), 10)
			b = int(_next(lines), 10)
			print(f'        [{r}, {g}, {b}],')
		print('    ], [')
	print('];')
	print()
