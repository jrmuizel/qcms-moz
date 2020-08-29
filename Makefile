CXXFLAGS=-march=core-avx2
QCMS_OBJS=iccread.o chain.o matrix.o transform.o transform_util.o transform-sse2.o transform-sse1.o transform-avx.o

PROGRAMS=test

all: $(PROGRAMS)

$(PROGRAMS): $(QCMS_OBJS)
