package com.jnigen.tests;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.junit.jupiter.api.Assertions.assertArrayEquals;

import org.junit.jupiter.api.Test;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.ValueSource;

public class Tests {

	// Passthrough tests

	@ParameterizedTest
	@ValueSource(ints = { -1, 0, 1, Integer.MAX_VALUE, Integer.MIN_VALUE })
	public void passthroughI32(int value) throws Exception {
		MyTestClass t = new MyTestClass();
		assertEquals(value, t.passthroughI32(value));
	}

	@ParameterizedTest
	@ValueSource(longs = { -1, 0, 1, Long.MAX_VALUE, Long.MIN_VALUE })
	public void passthroughI64(long value) throws Exception {
		MyTestClass t = new MyTestClass();
		assertEquals(value, t.passthroughI64(value));
	}

	@ParameterizedTest
	@ValueSource(floats = { -1f, 0f, 1f, Float.MAX_VALUE, Float.MIN_VALUE })
	public void passthroughF32(float value) throws Exception {
		MyTestClass t = new MyTestClass();
		assertEquals(value, t.passthroughF32(value));
	}

	@ParameterizedTest
	@ValueSource(doubles = { -1f, 0f, 1f, Double.MAX_VALUE, Double.MIN_VALUE })
	public void passthroughF64(double value) throws Exception {
		MyTestClass t = new MyTestClass();
		assertEquals(value, t.passthroughF64(value));
	}

	@ParameterizedTest
	@ValueSource(booleans = { false, true })
	public void passthroughBool(boolean value) throws Exception {
		MyTestClass t = new MyTestClass();
		assertEquals(value, t.passthroughBool(value));
	}

	@ParameterizedTest
	@ValueSource(strings = { "", "Hello World", "{\"some\":\"json\"}" })
	public void passthroughString(String value) throws Exception {
		MyTestClass t = new MyTestClass();
		assertEquals(value, t.passthroughString(value));
	}

	// Passthrough modify tests

	@ParameterizedTest
	@ValueSource(ints = { -1, 0, 1, Integer.MIN_VALUE })
	public void passthroughModI32(int value) throws Exception {
		MyTestClass t = new MyTestClass();
		assertEquals(value + 1, t.passthroughModI32(value));
	}

	@ParameterizedTest
	@ValueSource(longs = { -1, 0, 1, Long.MIN_VALUE })
	public void passthroughModI64(long value) throws Exception {
		MyTestClass t = new MyTestClass();
		assertEquals(value + 1, t.passthroughModI64(value));
	}

	@ParameterizedTest
	@ValueSource(floats = { -1f, 0f, 1f, Float.MIN_VALUE })
	public void passthroughModF32(float value) throws Exception {
		MyTestClass t = new MyTestClass();
		assertEquals(value + 1f, t.passthroughModF32(value));
	}

	@ParameterizedTest
	@ValueSource(doubles = { -1.0, 0.0, 1.0, Double.MIN_VALUE })
	public void passthroughModF64(double value) throws Exception {
		MyTestClass t = new MyTestClass();
		assertEquals(value + 1, t.passthroughModF64(value));
	}

	@ParameterizedTest
	@ValueSource(booleans = { false, true })
	public void passthroughModBool(boolean value) throws Exception {
		MyTestClass t = new MyTestClass();
		assertEquals(!value, t.passthroughModBool(value));
	}

	@ParameterizedTest
	@ValueSource(strings = { "", "Hello World", "{\"some\":\"json\"}" })
	public void passthroughModString(String value) throws Exception {
		MyTestClass t = new MyTestClass();
		assertEquals(value + "Mod", t.passthroughModString(value));
	}

	// panics / exceptions
	@Test
	public void panicShouldThrowException() {
		MyTestClass t = new MyTestClass();
		assertThrows(RuntimeException.class, () -> {
			t.shouldPanic();
		});
	}

	@Test
	public void panicShouldThrowExceptionResultF64() {
		MyTestClass t = new MyTestClass();
		assertThrows(RuntimeException.class, () -> {
			t.shouldPanicResultF64();
		});
	}

	@Test
	public void panicShouldThrowExceptionResultString() {
		MyTestClass t = new MyTestClass();
		assertThrows(RuntimeException.class, () -> {
			t.shouldPanicResultString();
		});
	}

	// Result return types
	@Test
	public void resultOk() throws Exception {
		MyTestClass t = new MyTestClass();
		assertEquals(10, t.returnsResultOk());
	}

	@Test
	public void resultErrThrows() {
		MyTestClass t = new MyTestClass();
		RuntimeException e = assertThrows(RuntimeException.class, () -> {
			t.returnsResultErrOkUnit();
		});
		assertEquals("Error Text", e.getMessage());
	}

	@Test
	public void resultErrThrowsOkBool() {
		MyTestClass t = new MyTestClass();
		RuntimeException e = assertThrows(RuntimeException.class, () -> {
			t.returnsResultErrOkBool();
		});
		assertEquals("Error Text", e.getMessage());
	}

	@Test
	public void resultErrThrowsOkString() {
		MyTestClass t = new MyTestClass();
		RuntimeException e = assertThrows(RuntimeException.class, () -> {
			t.returnsResultErrOkString();
		});
		assertEquals("Error Text", e.getMessage());
	}

	// custom public name
	@Test
	public void customPublicName() throws Exception {
		MyTestClass t = new MyTestClass();
		t.customPublicName();
	}

	// array of Strings
	@Test
	public void arrayOfStringsEmpty() throws Exception {
		MyTestClass t = new MyTestClass();
		String concatenated = t.stringVecArgument(new String[] {});
		assertEquals("", concatenated);
	}

	@Test
	public void arrayOfStringsSingle() throws Exception {
		MyTestClass t = new MyTestClass();
		String concatenated = t.stringVecArgument(new String[] { "one" });
		assertEquals("one", concatenated);
	}

	@Test
	public void arrayOfStringsMultiple() throws Exception {
		MyTestClass t = new MyTestClass();
		String concatenated = t.stringVecArgument(new String[] { "one", "two", "three" });
		assertEquals("onetwothree", concatenated);
	}

	@Test
	public void arrayOfBytesEmpty() throws Exception {
		MyTestClass t = new MyTestClass();
		String concatenated = t.bytesVecArgument(new byte[] {});
		assertEquals("[]", concatenated);
	}

	@Test
	public void arrayOfBytesSingle() throws Exception {
		MyTestClass t = new MyTestClass();
		String concatenated = t.bytesVecArgument(new byte[] { 8 });
		assertEquals("[8]", concatenated);
	}

	@Test
	public void arrayOfBytesMultiple() throws Exception {
		MyTestClass t = new MyTestClass();
		String concatenated = t.bytesVecArgument(new byte[] { 8, 16 });
		assertEquals("[8, 16]", concatenated);
	}

	@Test
	public void arrayBytesPassthrough() throws Exception {
		MyTestClass t = new MyTestClass();
		assertArrayEquals(new byte[] {}, t.bytesVecReturn(new byte[] {}));
		assertArrayEquals(new byte[] { 2 }, t.bytesVecReturn(new byte[] { 2 }));
		assertArrayEquals(new byte[] { 8, 16 }, t.bytesVecReturn(new byte[] { 8, 16 }));
	}

	@Test
	public void arrayBytesResultPassthrough() throws Exception {
		MyTestClass t = new MyTestClass();
		assertArrayEquals(new byte[] {}, t.bytesVecReturnResult(new byte[] {}));
		assertArrayEquals(new byte[] { 2 }, t.bytesVecReturnResult(new byte[] { 2 }));
		assertArrayEquals(new byte[] { 8, 16 }, t.bytesVecReturnResult(new byte[] { 8, 16 }));
	}

	@Test
	public void arrayBytesResultErr() throws Exception {
		MyTestClass t = new MyTestClass();
		assertThrows(RuntimeException.class, () -> {
			t.bytesVecReturnResultErr();
		});
	}

	@Test
	public void optionString() throws Exception {
		MyTestClass t = new MyTestClass();
		assertEquals("Some(\"\")", t.optionString(""));
		assertEquals("Some(\"hello\")", t.optionString("hello"));
		assertEquals("None", t.optionString(null));
	}

	@Test
	public void optionByteArray() throws Exception {
		MyTestClass t = new MyTestClass();
		assertEquals(0, t.optionByteArray(new byte[] {}));
		assertEquals(3, t.optionByteArray(new byte[] { 0, 1, 2 }));
		assertEquals(-1, t.optionByteArray(null));
	}

	@Test
	public void optionStringArray() throws Exception {
		MyTestClass t = new MyTestClass();
		assertEquals("Some([])", t.optionStringArray(new String[] {}));
		assertEquals("Some([\"one\", \"two\", \"three\"])",
				t.optionStringArray(new String[] { "one", "two", "three" }));
		assertEquals("None", t.optionStringArray(null));
	}

	@Test
	public void optionStringReturn() throws Exception {
		MyTestClass t = new MyTestClass();
		assertEquals("string", t.optionStringReturn(false));
		assertEquals(null, t.optionStringReturn(true));
	}
}
