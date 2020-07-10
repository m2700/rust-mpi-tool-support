CC=mpicc
CFLAGS=-L../../tool/target/debug/ -lmpi_test_tool2

all: test_app.c
	$(CC) $(CFLAGS) -o test_app test_app.c

clean:
	rm test_app
