#SRC_PATH=$(dir $(realpath $(firstword $(MAKEFILE_LIST))))
SRC_PATH=src/transformation/
LIB_PATH_DEBUG=target/debug/deps/
LIB_PATH_RELEASE=target/release/deps/

$(SRC_PATH)libopencv_resize.so: $(SRC_PATH)opencv_resize.o
	g++ -shared -o $(SRC_PATH)libopencv_resize.so $(SRC_PATH)opencv_resize.o `pkg-config --cflags --libs opencv4`



$(SRC_PATH)opencv_resize.o: $(SRC_PATH)opencv_resize.cpp
	g++ -Wall -fPIC -c $(SRC_PATH)opencv_resize.cpp -o $(SRC_PATH)opencv_resize.o `pkg-config --cflags --libs opencv4`

clean:
	rm $(SRC_PATH)opencv_resize.o $(SRC_PATH)libopencv_resize.so

install_debug: $(SRC_PATH)libopencv_resize.so
	cp $(SRC_PATH)libopencv_resize.so $(LIB_PATH_DEBUG) -f

install_release: $(SRC_PATH)libopencv_resize.so
	cp $(SRC_PATH)libopencv_resize.so $(LIB_PATH_RELEAS) -f
