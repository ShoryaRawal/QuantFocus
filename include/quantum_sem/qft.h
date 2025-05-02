/** 
 * @file qft.h
 * @brief Quantum Fourier Transform Transform Implementation for SEM image enhancement
 * 
 * This header provides declarations for Quantum Fourier Transform operations specialized for
 * SEM image processing. The implementation leverages quantum-inspirted principles to achieve 
 * superior frequency domain transformations compared to classical Fourier Transform approaches.
 * 
 * @Shorya Rawal
 * @2 May 2025
 */

#ifndef QUANTUM_SEM_QFT_H
#define QUANTUM_SEM_QFT_H

#include "core.h"

#ifdef __cplusplus
extern "C"{
#endif

/**
 * @brief Configuration for QFT operations
 */
typedef struct{
    uint32_t window_size;
    float phase_correction;
    uint8_t normalize_output;
    uint8_t apply_window_function;
    uint8_t window_function_type;
    uint8_t optimization_level;
    uint8_t use_parallel;
}QSEMQftConfig;

/**
 * @brief Initialize default QFT configuration
 * 
 * @param[out] config Pointer to config structure
 * @return QSEMStatus QSEM_SUCCESS on success, error codes otherwise
 */
QSEMStatus qsem_qft_config_init(QSEMQftConfig * config);

/**
 * @brief Apply 1D Quantum Fourier Transform to a vector of complex values
 * 
 * @param[in] context Framework context
 * @param[in] input Input complex vector
 * @param[out] output Output complex vector
 * @param[in] length Length of input/output vectors
 * @param[in] inverse Whether to perform inverse QFT (0 = forward, 1 = inverse)
 * @param[in] config Congiguration parameters (NULL for defaults)
 */

QSEMStatus qsem_qft_1d(QSEMContext * context, const QSEMComplex * input, QSEMComplex * output, uint32_t length, uint8_t inverse, const QSEMQftConfig * config);

/**
 * @brief Apply 2D Quantum Fourier Transform to a vector of complex values
 */
QSEMStatus qsem_qft_2d(QSEMContext * context, const QSEMComplex * input, QSEMComplex * output, uint32_t width, uint32_t height, uint8_t inverse, const QSEMQftConfig * config);

/**
 * @brief Apply QFT to real-valued image data
 */
QSEMStatus qsem_qft_image_to_frequency(QSEMContext * context, const QSEMImage * image, QSEMComplex * output, const QSEMQftConfig * config);

/**
 * @brief APply inverse QFT to frequency domain data and produce real-valued image
 */
QSEMStatus qsem_qft_frequency_to_image(QSEMContext * context, const QSEMComplex * input, QSEMImage * output, const QSEMQftConfig * config);

/**
 * @brief QFT-based frequency filter to image
 */
QSEMStatus qsem_qft_apply_filter(QSEMContext * context, const QSEMImage * input, QSEMImage * output,
                                void (* filter_func)(QSEMComplex * freq_data, uint32_t x, uint32_t y, uint32_t width, uint32_t height, void * user_data),
                                void * user_data, const QSEMQftConfig * config);
QSEMStatus qsem_qft_highpass_filter(QSEMContext * context, const QSEMImage * input, QSEMImage * output, float strength, const QSEMQftConfig * config);
QSEMStatus qsem_qft_lowpass_filter(QSEMContext * context, const QSEMImage * input, QSEMImage * output, float cutoff, const QSEMQftConfig * config);
QSEMStatus qsem_qft_bandpass_filter(QSEMContext * context, const QSEMImage * input, QSEMImage * output, float low_cutoff, float high_cutoff, const QSEMQftConfig * config);
QSEMStatus qsem_qft_adaptive_filter(QSEMContext * context, const QSEMImage * input, QSEMImage * output, const QSEMQftConfig * config);
QSEMStatus qsem_qft_enhance_contrast(QSEMContext * context, const QSEMImage * input, QSEMImage * output, float enhancement_factor, const QSEMQftConfig * config);
QSEMStatus qsem_qft_power_spectrum(QSEMContext * context, const QSEMComplex * input, QSEMImage * output, float * power_spectrum, uint32_t height, uint32_t width, uint8_t log_scale);
QSEMStatus qsem_qft_visualize(QSEMContext * context, const QSEMComplex * freq_data, QSEMImage * output, uint32_t height, uint32_t width, uint8_t log_scale);

#ifdef __cplusplus
}
#endif

#endif