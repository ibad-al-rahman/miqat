# Keep rules for consumers of miqat. miqat's UniFFI bindings call the Rust
# `libmiqat.so` through JNA, and JNA resolves Java fields/classes by name at the
# JNI boundary. R8/ProGuard renaming or stripping them breaks native calls at
# runtime (e.g. "Can't obtain peer field ID for class com.sun.jna.Pointer" or
# corrupted Structure layouts) — but only in minified builds, so debug works and
# release crashes. These rules are shipped in the AAR so every consumer inherits
# them automatically.

# --- JNA ---
# libjnidispatch.so's initIDs() looks up com.sun.jna.* fields by name; Structure
# subclasses map Java fields to native memory by name + declared order.
-keep class com.sun.jna.** { *; }
-keepclassmembers class com.sun.jna.** { *; }
-keep class * extends com.sun.jna.** { *; }
-keepclassmembers class * extends com.sun.jna.Structure {
    <fields>;
}
# JNA callbacks are invoked from native code by interface method name.
-keep interface * extends com.sun.jna.Callback { *; }
-dontwarn java.awt.**

# --- UniFFI-generated bindings for miqat ---
# RustBuffer/ForeignBytes/UniffiRustCallStatus etc. are JNA Structure/Callback
# subclasses; their field names and order are the FFI contract with libmiqat.so.
-keep class org.ibadalrahman.miqat.** { *; }
-keepclassmembers class org.ibadalrahman.miqat.** { *; }
