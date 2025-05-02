#ifndef QUANTUM_SEM_CORE_H
#define QUANTUM_SEM_CORE_H

#include<stdint.h>
#include<stdlib.h>

typedef struct{
	uint32_t width;
	uint32_t height;
	uint8_t channels;

	float * data;

	uint32_t stride;
} QSEMImage;

typedef struct{
	float real;
	float image;
} QSEMComplex;

typedef enum{
	QSEM_SUCCESS = 0,
	QSEM_ERROR_MEMORY,
	QSEM_ERROR_IO,
	QSEM_ERROR_INVALID_PARAM,
	QSEM_ERROR_HARDWARE,
	QSEM_ERROR_ALGORITHM
} QSEMStatus;

typedef struct{
	uint8_t num_threads;
	uint8_t use_gpu;
	
	float noise_threashold;

	uint32_t qft_window_size;
	uint8_t tensor_decomp_rank;
} QSEMConfig;

typedef struct{
	QSEMConfig config;
	void * memory_pool;
	void * hardware_handle;
	void * private_data;
} QSEMContext;

QSEMStatus qsem_init(QSEMContext** context, const QSEMConfig* config);
QSEMStatus qsem_finalize(QSEMContext* context);
const char * qsem_status_string(QSEMStatus status);

#endif
