CFLAGS=-march=core-avx2 -fno-blocks
QCMS_OBJS=iccread.o chain.o matrix.o transform.o transform_util.o transform-sse2.o transform-sse1.o transform-avx.o

PROGRAMS=test

all: $(PROGRAMS)

$(PROGRAMS): $(QCMS_OBJS)

clean:
	rm -f *.o
