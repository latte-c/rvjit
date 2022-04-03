class RegisterDefs:
    def __init__(self, filename) -> None:
        self.file = open(filename, 'w')

    def close(self):
        self.file.close()

    def writeln(self, content):
        self.file.write(content + '\n')

    def write_general(self):
        for i in range(0, 32):
            self.writeln(f'pub const X{i}: u8 = {i};')

    def write_alias(self, name: str, i):
        self.writeln(f'pub const {name.upper()}: u8 = X{i};')


regs = RegisterDefs('../src/register.rs')
regs.write_general()
regs.writeln('\n// alias registers')
regs.write_alias('zero', 0)
regs.write_alias('ra', 1)
regs.write_alias('sp', 2)
regs.write_alias('gp', 3)
regs.write_alias('tp', 4)
for i in range(0, 3):
    regs.write_alias(f't{i}', i + 5)
regs.write_alias('s0', 8)
regs.write_alias('fp', 8)
regs.write_alias('s1', 9)
for i in range(0, 8):
    regs.write_alias(f'a{i}', i + 10)
for i in range(2, 12):
    regs.write_alias(f's{i}', i + 16)
for i in range(3, 7):
    regs.write_alias(f't{i}', i + 25)

regs.close()
