# SPDX-FileCopyrightText: 2023 Baking Bad <hello@bakingbad.dev>
#
# SPDX-License-Identifier: MIT

trace-e2e:
	RUST_LIB_BACKTRACE=1 cargo test --jobs 1 --no-fail-fast --test e2e --features trace -- --nocapture --test-threads=1 $(NAME)
