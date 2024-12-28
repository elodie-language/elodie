#include "../unit-test.h"
#include "core/val/val-api.h"
#include "../../../src/type/type-impl.h"

TEST(type_node, ok)
{
	auto tm = mem_test_new_default (1024);

	auto result = type_node_new (MEM(tm), 28, 11, VAL_STR_VIEW ("ident"));
	ASSERT_EQ(28, result->id);
	ASSERT_EQ(11, result->base_id);
	ASSERT_TRUE(VAL_EQ (result->ident, "ident"));

	type_node_free (result, MEM(tm));
	mem_test_verify (tm);
	mem_test_free (tm);
}

TEST(type_edge, ok)
{
	auto tm = mem_test_new_default (1024);

	auto genesis = type_edge_new (MEM(tm), 0, 0, nullptr);

	auto result = type_edge_new (MEM(tm), 2, 1, genesis);
	ASSERT_EQ(1, result->base_id);
	ASSERT_EQ(2, result->type_id);
	ASSERT_EQ(genesis, result->prev);

	type_edge_free (result, MEM(tm));
	type_edge_free (genesis, MEM(tm));
	mem_test_verify (tm);
	mem_test_free (tm);
}