import struct
Position = (int, int, int)

def copy_dictionary(odict):
    return {k: v for (k, v) in odict.items()}

class Game:
    def __init__(self):
        self.cubes = {}
    def set_cube(self, position: Position, v: bool):
        (x, y, z) = position
        self.cubes[(x, y, z)] = v
    def get_cube(self, position: Position) -> bool:
        if position in self.cubes:
            return self.cubes[position]
        return False
    def bounds(self) -> (Position, Position):
        min_x = min([x for (x, y, z) in self.cubes.keys()])
        min_y = min([y for (x, y, z) in self.cubes.keys()])
        min_z = min([z for (x, y, z) in self.cubes.keys()])

        max_x = max([x for (x, y, z) in self.cubes.keys()])
        max_y = max([y for (x, y, z) in self.cubes.keys()])
        max_z = max([z for (x, y, z) in self.cubes.keys()])

        min_pos = (min_x, min_y, min_z)
        max_pos = (max_x, max_y, max_z)
        return (min_pos, max_pos)
    def neighbours(self, center):
        acc = 0
        for pos in block_gen(vector_constant_add(center, -1), vector_constant_add(center, 1)):
            if pos == center:
                continue
            if self.get_cube(pos):
                acc += 1
        return acc
    def round(self):
        old = Game()
        old.cubes = copy_dictionary(self.cubes)
        (min_pos, max_pos) = self.bounds()
        for pos in block_gen(vector_constant_add(min_pos, -1), vector_constant_add(max_pos, 1)):
            v = old.get_cube(pos)
            n = old.neighbours(pos)
            if v and not (n == 2 or n == 3):
                self.set_cube(pos, False)
            if not v and n == 3:
                self.set_cube(pos, True)
    def save(self, filename):
        with open(filename, "wb") as f:
            for p, c in self.cubes.items():
                if c:
                    data = struct.pack("<iii", p[0], p[1], p[2])
                    f.write(data)

def block_gen(min_pos, max_pos):
    (min_x, min_y, min_z) = min_pos
    (max_x, max_y, max_z) = max_pos
    for x in range(min_x, max_x+1):
        for y in range(min_y, max_y+1):
            for z in range(min_z, max_z+1):
                yield (x, y, z)

def vector_constant_add(vec, const):
    return (vec[0]+const, vec[1]+const, vec[2]+const)


with open("input.txt", "r") as f:
    def_state = [a.strip() for a in f.readlines()]
game = Game()
for y, yv in enumerate(def_state):
        for x, xv in enumerate(yv):
            if xv == "#":
                game.set_cube((x, y, 0), True)
game.save("snapshots/0.bin")
for i in range(6):
    game.round()
    game.save("snapshots/%d.bin" % (i+1))