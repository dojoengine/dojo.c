#ifndef Pagination_H
#define Pagination_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DojoError.d.h"
#include "OrderBy.d.h"
#include "PaginationDirection.d.h"

#include "Pagination.d.h"






Pagination* Pagination_new(void);

typedef struct Pagination_set_cursor_result {union { DojoError* err;}; bool is_ok;} Pagination_set_cursor_result;
Pagination_set_cursor_result Pagination_set_cursor(Pagination* self, DiplomatStringView cursor);

void Pagination_set_limit(Pagination* self, uint32_t limit);

void Pagination_set_direction(Pagination* self, PaginationDirection direction);

void Pagination_add_order_by(Pagination* self, const OrderBy* order_by);

void Pagination_destroy(Pagination* self);





#endif // Pagination_H
