/* api.h --- 
 * 
 * Filename: api.h
 * Author: Louise <louise>
 * Created: Mon Sep 30 16:17:43 2019 (+0200)
 * Last-Updated: Fri Oct  4 12:50:13 2019 (+0200)
 *           By: Louise <louise>
 */
#ifndef RGBA_API_H
#define RGBA_API_H
#include <stdbool.h>
#include <stdint.h>

// Structs
struct CoreInfo {
    char * name;
    char * console;
    char * version;
    char * author;
};

// General fonctions
extern bool            rgba_core_is_file(char filename);
extern struct CoreInfo rgba_core_get_coreinfo();

// Instance functions
// Creates a new instance. Should theorically be able to be called an unlimited number of times,
// and returns a pointer to the instance, which will be sent with each subsequent function calls
extern void * rgba_core_init();

// Frees memory associated with an instance
extern void   rgba_core_deinit(void * data);

// Run the core either until the end of the frame, or until the next breakpoint
extern void   rgba_core_run(void * data);

// Load extra files, such as the BIOS for cores who need one
extern void   rgba_core_load_extra(void * data, char * load_name, char * filename);

// Load the actual ROM file
extern void   rgba_core_load_rom(void * data, char * filename);

// Returns NULL if the core is ready to run, a string to an error message in another case.
extern char * rgba_core_finish(void * data);

// Set callback
extern void   rgba_core_set_cb_present_frame(void * data, FrontendCallbackVideoPresentFrame cb);
extern void   rgba_core_set_cb_queue_sample(void * data, FrontendCallbackAudioQueueSamples cb);

// Callback types
typedef (void)(*FrontendCallbackVideoPresentFrame)(uint32_t * frame);
typedef (void)(*FrontendCallbackAudioQueueSamples)(ssize_t array_size, int16_t array);

#endif
