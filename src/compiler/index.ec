
fs.create_directory('/tmp/target')
fs.create_directory('/tmp/target/debug')
fs.create_file('/tmp/target/debug/main.c')
fs.write_to_file('/tmp/target/debug/main.c', '#include <stdio.h>')
fs.write_to_file('/tmp/target/debug/main.c', 'int main(void) {')
fs.write_to_file('/tmp/target/debug/main.c', '     printf("ElodiE!\n");')
fs.write_to_file('/tmp/target/debug/main.c', '    return 0;')
fs.write_to_file('/tmp/target/debug/main.c', '}')