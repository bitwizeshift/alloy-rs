# GLEW

This is a Rust-wrapper around the [OpenGL Extension Wrangler (GLEW)] package and
OpenGL 3/4 itself.

[OpenGL Extension Wrangler (GLEW)]: https://glew.sourceforge.net/

## Conventions

OpenGL has an idiosynchratic way of testing for errors, often making use of
utilities like [`glGetError`] to get if the last call has failed. To account
for this, this library contains 3 different conventions for function naming.

* `*_unchecked`: `unsafe` functions that _should_ test for an error in most
  cases, but are left untested to avoid the overhead for when the caller already
  knows an error-condition has been met.
* `*` (unprefixed): safe functions that either test for errors implicitly, or
  don't bother need to test at all since the functionality is reasonably safe.
* `*_checked`: safe functions that _don't need_ to test for errors, but are
  still _technically_ fallible and could be checked. Often this is useful for
  detecting failures after cases like a [`glGet*`] -- which is normally
  reasonably safe to assume will succeed, but may fail from a bad context
  profile.

[`glGet*`]: https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGet.xhtml
[`glGetError`]: https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glGetError.xhtml
