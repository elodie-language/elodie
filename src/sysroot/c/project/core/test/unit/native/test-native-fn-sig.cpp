#include "../unit-test.h"
#include "core/native/native-fn-sig.h"

TEST(native_fn_signature_ident_allocate, ok)
{
	auto tm = mem_test_new_default (512);

	auto result = native_fn_signature_ident_allocate (MEM(tm), STRING_VIEW("some_package::some_fn"));
	ASSERT_TRUE(string_equal_c_str (result->ident, "some_package::some_fn"));
	ASSERT_EQ(12, result->marker);

	native_fn_signature_ident_deallocate (result, MEM(tm));
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(native_fn_signature_ident_init, ok)
{
	auto tm = mem_test_new_default (512);

	struct native_fn_signature_ident test_instance{};
	native_fn_signature_ident_init (&test_instance, MEM(tm), STRING_VIEW("some_package::some_fn"));
	ASSERT_TRUE(string_equal_c_str (test_instance.ident, "some_package::some_fn"));
	ASSERT_EQ(12, test_instance.marker);

	native_fn_signature_ident_reset (&test_instance, MEM(tm));
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(native_fn_signature_ident, ok)
{
	auto tm = mem_test_new_default (512);

	struct native_fn_signature_ident test_instance{};
	native_fn_signature_ident_init (&test_instance, MEM(tm), STRING_VIEW("some_package::some_fn"));

	ASSERT_TRUE(STRING_VIEW_EQUAL (native_fn_signature_ident_package_ident (test_instance), STRING_VIEW ("some_package")));
	ASSERT_TRUE(STRING_VIEW_EQUAL (native_fn_signature_ident_fn_ident (test_instance), STRING_VIEW ("some_fn")));
	ASSERT_TRUE(STRING_VIEW_EQUAL (native_fn_signature_ident (test_instance), STRING_VIEW ("some_package::some_fn")));

	native_fn_signature_ident_reset (&test_instance, MEM(tm));
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(native_fn_signature_ident_deallocate_safe, ok)
{
	auto tm = mem_test_new_default (512);

	auto test_instance = native_fn_signature_ident_allocate (MEM(tm), STRING_VIEW("some_package::some_fn"));
	native_fn_signature_ident_deallocate_safe (&test_instance, MEM(tm));
	ASSERT_TRUE(test_instance == nullptr);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(native_fn_signature_param_allocate, ok)
{
	auto tm = mem_test_new_default (512);

	auto result = native_fn_signature_param_allocate (MEM(tm), STRING_VIEW("some_parameter"), type_string);
	ASSERT_TRUE(type_equal (result->type, type_string));
	ASSERT_TRUE(string_equal_c_str (result->ident, "some_parameter"));
	ASSERT_EQ(result->next, nullptr);

	native_fn_signature_param_deallocate_safe (&result, MEM(tm));
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(native_fn_signature_param_init, ok)
{
	auto tm = mem_test_new_default (512);

	struct native_fn_signature_param test_instance{};
	native_fn_signature_param_init (&test_instance, MEM(tm), STRING_VIEW("some_parameter"), type_string);
	ASSERT_TRUE(type_equal (test_instance.type, type_string));
	ASSERT_TRUE(string_equal_c_str (test_instance.ident, "some_parameter"));
	ASSERT_EQ(test_instance.next, nullptr);

	native_fn_signature_param_reset (&test_instance, MEM(tm));
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(native_fn_signature_param_append, append_to_single_param)
{
	auto tm = mem_test_new_default (512);

	struct native_fn_signature_param test_instance{};
	native_fn_signature_param_init (&test_instance, MEM(tm), STRING_VIEW("some_parameter"), type_string);

	struct native_fn_signature_param param_to_append{};
	native_fn_signature_param_init (&param_to_append, MEM(tm), STRING_VIEW("param_to_append"), type_string);

	native_fn_signature_param_append (&test_instance, &param_to_append);
	ASSERT_EQ(test_instance.next, &param_to_append);

	native_fn_signature_param_reset (&test_instance, MEM(tm));
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(native_fn_signature_param_append, append_to_multiple_params)
{
	auto tm = mem_test_new_default (512);

	struct native_fn_signature_param test_instance{};
	ASSERT_EQ(1, native_fn_signature_param_count (&test_instance));
	native_fn_signature_param_init (&test_instance, MEM(tm), STRING_VIEW("some_parameter"), type_string);

	struct native_fn_signature_param another_param{};
	native_fn_signature_param_init (&another_param, MEM(tm), STRING_VIEW("another_parma"), type_string);
	native_fn_signature_param_append (&test_instance, &another_param);
	ASSERT_EQ(2, native_fn_signature_param_count (&test_instance));

	struct native_fn_signature_param param_to_append{};
	native_fn_signature_param_init (&param_to_append, MEM(tm), STRING_VIEW("param_to_append"), type_string);
	native_fn_signature_param_append (&test_instance, &param_to_append);
	ASSERT_EQ(3, native_fn_signature_param_count (&test_instance));

	ASSERT_EQ(test_instance.next, &another_param);
	ASSERT_EQ(another_param.next, &param_to_append);

	native_fn_signature_param_reset (&test_instance, MEM(tm));
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(native_fn_signature_param_deallocate_safe, ok)
{
	auto tm = mem_test_new_default (512);
	auto test_instance = native_fn_signature_param_allocate (MEM(tm), STRING_VIEW("some_parameter"), type_string);
	native_fn_signature_param_append (test_instance, native_fn_signature_param_allocate (MEM(tm), STRING_VIEW("param_to_append"), type_string));

	native_fn_signature_param_deallocate_safe (&test_instance, MEM(tm));
	ASSERT_TRUE(test_instance == nullptr);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(native_fn_signature_result_allocate, ok)
{
	auto tm = mem_test_new_default (512);

	auto result = native_fn_signature_result_allocate (MEM(tm), type_number);
	ASSERT_TRUE(type_equal (result->type, type_number));

	native_fn_signature_result_deallocate_safe (&result, MEM(tm));
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(native_fn_signature_result_init, ok)
{
	auto tm = mem_test_new_default (512);

	struct native_fn_signature_result test_instance{};
	native_fn_signature_result_init (&test_instance, MEM(tm), type_string);
	ASSERT_TRUE(type_equal (test_instance.type, type_string));

	native_fn_signature_result_reset (&test_instance, MEM(tm));
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(native_fn_signature_result_deallocate_safe, ok)
{
	auto tm = mem_test_new_default (512);
	auto test_instance = native_fn_signature_result_allocate (MEM(tm), type_string);

	native_fn_signature_result_deallocate_safe (&test_instance, MEM(tm));
	ASSERT_TRUE(test_instance == nullptr);

	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(native_fn_signature_allocate, ok)
{
	auto tm = mem_test_new_default (512);
	auto sig_param = native_fn_signature_param_allocate (MEM(tm), STRING_VIEW("some_parameter"), type_string);

	struct native_fn_signature_result sig_result{};
	native_fn_signature_result_init (&sig_result, MEM(tm), type_string);

	struct native_fn_signature_ident sig_ident{};
	native_fn_signature_ident_init (&sig_ident, MEM(tm), STRING_VIEW("package_ident::fn_ident"));

	auto result = native_fn_signature_allocate (MEM(tm), sig_ident, sig_param, sig_result);

	ASSERT_TRUE(STRING_VIEW_EQUAL (native_fn_signature_ident_package_ident (result->ident), "package_ident"));
	ASSERT_TRUE(STRING_VIEW_EQUAL (native_fn_signature_ident_fn_ident (result->ident), "fn_ident"));
	ASSERT_EQ(result->params, sig_param);
	ASSERT_TRUE(type_equal (result->result.type, type_string));
	ASSERT_EQ(16037384996350040278LLU, result->hash.value);

	native_fn_signature_deallocate_safe (&result, MEM(tm));
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(native_fn_signature_init, ok)
{
	auto tm = mem_test_new_default (512);
	auto sig_param = native_fn_signature_param_allocate (MEM(tm), STRING_VIEW("some_parameter"), type_string);

	struct native_fn_signature_result sig_result{};
	native_fn_signature_result_init (&sig_result, MEM(tm), type_string);

	struct native_fn_signature_ident sig_ident{};
	native_fn_signature_ident_init (&sig_ident, MEM(tm), STRING_VIEW("package_ident::fn_ident"));

	struct native_fn_signature test_instance{};
	native_fn_signature_init (&test_instance, MEM(tm), sig_ident, sig_param, sig_result);

	ASSERT_TRUE(STRING_VIEW_EQUAL (native_fn_signature_ident_package_ident (test_instance.ident), "package_ident"));
	ASSERT_TRUE(STRING_VIEW_EQUAL (native_fn_signature_ident_fn_ident (test_instance.ident), "fn_ident"));
	ASSERT_EQ(test_instance.params, sig_param);
	ASSERT_TRUE(type_equal (test_instance.result.type, type_string));
	ASSERT_EQ(16037384996350040278LLU, test_instance.hash.value);

	native_fn_signature_reset (&test_instance, MEM(tm));
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(native_fn_sigdeallocate_safe, ok)
{
	auto tm = mem_test_new_default (512);

	struct native_fn_signature_result sig_result{};
	native_fn_signature_result_init (&sig_result, MEM(tm), type_string);

	struct native_fn_signature_ident sig_ident{};
	native_fn_signature_ident_init (&sig_ident, MEM(tm), STRING_VIEW("package_ident::fn_ident"));

	auto test_instance = native_fn_signature_allocate (
		MEM(tm),
		sig_ident,
		native_fn_signature_param_allocate (MEM(tm), STRING_VIEW("some_parameter"), type_string),
		sig_result
	);

	native_fn_signature_deallocate_safe (&test_instance, MEM(tm));
	ASSERT_TRUE(test_instance == nullptr);

	mem_test_verify (tm);
	mem_test_free (tm);
}
