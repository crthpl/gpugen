// Generated by jextract

package com.crthpl.gpugen.rust;

import java.lang.invoke.MethodHandle;
import java.lang.invoke.VarHandle;
import java.nio.ByteOrder;
import java.lang.foreign.*;
import static java.lang.foreign.ValueLayout.*;
final class constants$2 {

    // Suppresses default constructor, ensuring non-instantiability.
    private constants$2() {}
    static final MethodHandle const$0 = RuntimeHelper.downcallHandle(
        "get_min_height",
        constants$1.const$4
    );
    static final MethodHandle const$1 = RuntimeHelper.downcallHandle(
        "get_sea_level",
        constants$1.const$4
    );
    static final FunctionDescriptor const$2 = FunctionDescriptor.of(RuntimeHelper.POINTER,
        RuntimeHelper.POINTER,
        JAVA_INT,
        JAVA_INT,
        JAVA_INT
    );
    static final MethodHandle const$3 = RuntimeHelper.downcallHandle(
        "get_debug_text",
        constants$2.const$2
    );
    static final MethodHandle const$4 = RuntimeHelper.downcallHandle(
        "free_generator",
        constants$0.const$5
    );
    static final MethodHandle const$5 = RuntimeHelper.downcallHandle(
        "free_chunk",
        constants$0.const$5
    );
}


