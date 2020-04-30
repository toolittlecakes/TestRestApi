//! C++ code that uses opencv functions for load image data,
//! resize it and store by rust_callback to out_ptr
#include <stdio.h>
#include <vector>
#include <inttypes.h>

#include <opencv2/opencv.hpp>
#include <opencv2/imgcodecs.hpp>


extern "C" {

typedef void (*rust_callback)(void * /* rust Vec*/, void * /*cpp vector data*/, size_t /*cpp vector size*/);
void *out_ptr = nullptr;
rust_callback store = nullptr;

int32_t register_output(void *output, rust_callback store_function) {
    if (output == nullptr || store_function == nullptr) {
        return -1;
    }
    out_ptr = output;
    store = store_function;
    return 0;
}

int32_t resize(void *in_ptr, int32_t in_size, int32_t num_rows, int32_t num_cols) {
//  unregistered output
    if (out_ptr == nullptr || store == nullptr) {
        return -1;
    }
//  invalid input
    if (in_ptr == nullptr || in_size <= 0 || num_rows <= 0 || num_cols <= 0) {
        return -2;
    }

    cv::Mat in_m{1, in_size, CV_8UC1, in_ptr};
    cv::InputArray in_a{in_m};

    try {
        auto src = cv::imdecode(in_a, cv::IMREAD_COLOR);

        if (src.data == nullptr || src.size().empty()) {
            return -3;
        }
        cv::Mat dst_m{num_rows, num_cols, CV_8UC1, cv::Scalar(0.)};

        cv::InputOutputArray dst{dst_m};
        cv::resize(src, dst, dst_m.size());
        std::vector <uint8_t> buff{};
        cv::imencode(".jpg", dst, buff);

        store(out_ptr, buff.data(), buff.size());
    }
    catch(...) {
        return -4;
    }
    return 0;

}
}