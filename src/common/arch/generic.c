/*
 *  ----------------------------------
 * |         libmem - by rdbo         |
 * |      Memory Hacking Library      |
 *  ----------------------------------
 */

/*
 * Copyright (C) 2025    Rdbo
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License version 3
 * as published by the Free Software Foundation.
 * 
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 * 
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

#include "arch.h"
#include <libmem/libmem.h>
#include <stdio.h>
#include <memory.h>

lm_arch_t
get_architecture()
{
	return LM_ARCH_GENERIC;
}

lm_size_t
generate_hook_payload(lm_address_t from, lm_address_t to, lm_size_t bits, lm_byte_t **payload_out)
{
	return 0;
}

lm_size_t
generate_no_ops(lm_byte_t *buf, lm_size_t size)
{
	return 0;
}
