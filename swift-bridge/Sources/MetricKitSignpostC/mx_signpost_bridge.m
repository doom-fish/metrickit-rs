#include "mx_signpost_bridge.h"

#include <MetricKit/MXSignpost_Private.h>
#include <mach-o/getsect.h>
#include <mach-o/loader.h>
#include <os/signpost.h>
#include <os/trace_base.h>
#include <stdbool.h>
#include <string.h>

static void mx_metrickit_emit_with_name_impl(
    void *dso,
    os_log_t log,
    os_signpost_type_t type,
    os_signpost_id_t signpost_id,
    const char *name,
    const char *format,
    uint8_t *buffer,
    uint32_t size
) {
    _os_signpost_emit_with_name_impl(dso, log, type, signpost_id, name, format, buffer, size);
}

static bool mx_metrickit_should_emit(os_log_t log, os_signpost_id_t signpost_id, const char *name) {
    return log != NULL
        && name != NULL
        && name[0] != '\0'
        && signpost_id != OS_SIGNPOST_ID_NULL
        && signpost_id != OS_SIGNPOST_ID_INVALID
        && os_signpost_enabled(log);
}

bool mx_metrickit_signpost_name_is_literal(const char *name) {
    if (name == NULL || name[0] == '\0') {
        return false;
    }

    unsigned long size = 0;
    const uint8_t *text = getsegmentdata((const struct mach_header_64 *)&__dso_handle, "__TEXT", &size);
    if (text == NULL || size == 0) {
        return false;
    }

    uintptr_t start = (uintptr_t)text;
    uintptr_t end = start + size;
    uintptr_t address = (uintptr_t)name;
    if (address < start || address >= end) {
        return false;
    }

    size_t available = (size_t)(end - address);
    return strnlen(name, available) < available;
}

uint64_t mx_metrickit_signpost_make_id(os_log_t log) {
    if (log == NULL) {
        return 0;
    }
    return (uint64_t)os_signpost_id_generate(log);
}

void mx_metrickit_signpost_event_emit(os_log_t log, uint64_t signpost_id, const char *name) {
    os_signpost_id_t cast_signpost_id = (os_signpost_id_t)signpost_id;
    if (!mx_metrickit_should_emit(log, cast_signpost_id, name)) {
        return;
    }

    OS_LOG_CALL_WITH_FORMAT(
        mx_metrickit_emit_with_name_impl,
        (&__dso_handle, log, OS_SIGNPOST_EVENT, cast_signpost_id, name),
        _MXSIGNPOST_METRICS_SNAPSHOT_FORMAT,
        _MXSignpostMetricsSnapshot()
    );
}

void mx_metrickit_signpost_interval_begin(os_log_t log, uint64_t signpost_id, const char *name) {
    os_signpost_id_t cast_signpost_id = (os_signpost_id_t)signpost_id;
    if (!mx_metrickit_should_emit(log, cast_signpost_id, name)) {
        return;
    }

    OS_LOG_CALL_WITH_FORMAT(
        mx_metrickit_emit_with_name_impl,
        (&__dso_handle, log, OS_SIGNPOST_INTERVAL_BEGIN, cast_signpost_id, name),
        _MXSIGNPOST_METRICS_SNAPSHOT_FORMAT,
        _MXSignpostMetricsSnapshot()
    );
}

void mx_metrickit_signpost_animation_interval_begin(os_log_t log, uint64_t signpost_id, const char *name) {
    os_signpost_id_t cast_signpost_id = (os_signpost_id_t)signpost_id;
    if (!mx_metrickit_should_emit(log, cast_signpost_id, name)) {
        return;
    }

    OS_LOG_CALL_WITH_FORMAT(
        mx_metrickit_emit_with_name_impl,
        (&__dso_handle, log, OS_SIGNPOST_INTERVAL_BEGIN, cast_signpost_id, name),
        "isAnimation=YES" _MXSIGNPOST_METRICS_SNAPSHOT_FORMAT,
        _MXSignpostMetricsSnapshot()
    );
}

void mx_metrickit_signpost_interval_end(os_log_t log, uint64_t signpost_id, const char *name) {
    os_signpost_id_t cast_signpost_id = (os_signpost_id_t)signpost_id;
    if (!mx_metrickit_should_emit(log, cast_signpost_id, name)) {
        return;
    }

    OS_LOG_CALL_WITH_FORMAT(
        mx_metrickit_emit_with_name_impl,
        (&__dso_handle, log, OS_SIGNPOST_INTERVAL_END, cast_signpost_id, name),
        _MXSIGNPOST_METRICS_SNAPSHOT_FORMAT,
        _MXSignpostMetricsSnapshot()
    );
}
