#include <assert.h>
#include <pulse/pulseaudio.h>
#include <signal.h>
#include <stdio.h>
#include <pthread.h>
#include <stdlib.h>

#define UNUSED(x) (void)(x)

float fl = 0.0;
int muted = 0;
int thread_status = -1;

pa_mainloop *mainloop = NULL;
pa_mainloop_api *mainloop_api = NULL;
pa_context *context = NULL;

float get_vol() { return fl; }
int get_muted() { return muted; }
int get_thread_status() { return thread_status; }

void vol_stop() {
    printf("VOL EXIT\n");
    if (mainloop_api) {
        mainloop_api->quit(mainloop_api, 0);
    }
}

void sink_info_callback(pa_context *c, const pa_sink_info *i, int eol, void *userdata) {
    UNUSED(c);
    UNUSED(eol);
    UNUSED(userdata);
    if (i) {
        float volume = (float)pa_cvolume_avg(&(i->volume)) / (float)PA_VOLUME_NORM;
        muted = i->mute;
        fl = volume * 100.0f;
        if (fl > 100) {
            system("/usr/bin/pactl set-sink-volume @DEFAULT_SINK@ 100%");
            fl = 100.0f;
        }
    }
}

void server_info_callback(pa_context *c, const pa_server_info *i, void *userdata) {
    pa_context_get_sink_info_by_name(c, i->default_sink_name, sink_info_callback, userdata);
}

void subscribe_callback(pa_context *c, pa_subscription_event_type_t type, uint32_t idx, void *userdata) {
    UNUSED(type);
    pa_operation *op = pa_context_get_sink_info_by_index(c, idx, sink_info_callback, userdata);
    if (op) pa_operation_unref(op);
}

void context_state_callback(pa_context *c, void *userdata) {
    switch (pa_context_get_state(c)) {
    case PA_CONTEXT_READY:
        pa_context_get_server_info(c, server_info_callback, userdata);
        pa_context_set_subscribe_callback(c, subscribe_callback, userdata);
        pa_context_subscribe(c, PA_SUBSCRIPTION_MASK_SINK, NULL, NULL);
        break;
    case PA_CONTEXT_TERMINATED:
        mainloop_api->quit(mainloop_api, 0);
        break;
    case PA_CONTEXT_FAILED:
    default:
        break;
    }
}

void *run(void *arg) {
    UNUSED(arg);
    thread_status = 1;
    pa_mainloop_run(mainloop, NULL);
    return NULL;
}

void vol_thread_start() {
    mainloop = pa_mainloop_new();
    mainloop_api = pa_mainloop_get_api(mainloop);
    
    context = pa_context_new(mainloop_api, "PA");
    pa_context_connect(context, NULL, PA_CONTEXT_NOAUTOSPAWN, NULL);
    pa_context_set_state_callback(context, context_state_callback, NULL);
    
    pthread_t thread_id;
    pthread_create(&thread_id, NULL, run, NULL);
    
    // Wait for the thread to finish
    // pthread_join(thread_id, NULL);

    // // Cleanup
    // if (context) {
    //     pa_context_disconnect(context);
    //     pa_context_unref(context);
    // }
    // if (mainloop) {
    //     pa_mainloop_free(mainloop);
    // }
}

// int main() {
//     vol_thread_start();
//     return 0;
// }
