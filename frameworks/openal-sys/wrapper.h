/// This wrapper header is used by Rust's bindgne to generate the appropriate
/// symbols for using OpenAL.
#ifndef OPENAL_WRAPPER_H
#define OPENAL_WRAPPER_H

#if __APPLE__
#  include <OpenAL/al.h>
#  include <OpenAL/alc.h>
#else
#  include <AL/al.h>
#  include <AL/alc.h>
#endif

#endif /* OPENAL_WRAPPER_H */
