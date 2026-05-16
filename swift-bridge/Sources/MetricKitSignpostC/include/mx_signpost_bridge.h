#pragma once

#include <stdint.h>
#include <os/log.h>

#ifdef __cplusplus
extern "C" {
#endif

uint64_t mx_metrickit_signpost_make_id(os_log_t log);
void mx_metrickit_signpost_event_emit(os_log_t log, uint64_t signpost_id, const char *name);
void mx_metrickit_signpost_interval_begin(os_log_t log, uint64_t signpost_id, const char *name);
void mx_metrickit_signpost_animation_interval_begin(os_log_t log, uint64_t signpost_id, const char *name);
void mx_metrickit_signpost_interval_end(os_log_t log, uint64_t signpost_id, const char *name);

#ifdef __cplusplus
}
#endif
