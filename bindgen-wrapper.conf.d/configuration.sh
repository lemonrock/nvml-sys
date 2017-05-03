# This file is part of nvml-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml-sys/master/COPYRIGHT. No part of nvml-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
# Copyright © 2016 The developers of nvml-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml-sys/master/COPYRIGHT.


bindingsName='nvml'
rootIncludeFileName='nvml.h'
macosXHomebrewPackageNames=''
alpineLinuxPackageNames='rsync make gcc linux-headers libunwind-dev linux-grsec-dev'
clangAdditionalArguments='-DPMEMOBJ_DIRECT_NON_INLINE'
headersFolderPath="$homeFolder"/compile-nvml.conf.d/temporary/usr/include
libFolderPath="$homeFolder"/compile-nvml.conf.d/temporary/usr/lib
link='nvml'
link_kind='static-nobundle'

final_chance_to_tweak()
{
	sed -i \
		-e 's/\*mut __jmp_buf_tag/jmp_buf/g' \
		"$outputFolderPath"/functions/obj_tx.rs \
		"$outputFolderPath"/functions/setjmp.rs

	cat >"$outputFolderPath"/constants/POBJ_FLAG.rs <<-EOF
		const POBJ_FLAG_ZERO: u64 = 1 << 0;
		const POBJ_FLAG_NO_FLUSH: u64 = 1 << 0;
	EOF

	cat >"$outputFolderPath"/constants/POBJ_XALLOC.rs <<-EOF
		pub const POBJ_XALLOC_ZERO: u64 = POBJ_FLAG_ZERO;
		pub const POBJ_XALLOC_NO_FLUSH: u64 = POBJ_FLAG_NO_FLUSH;
		pub const POBJ_XALLOC_VALID_FLAGS: u64 = (POBJ_XALLOC_ZERO | POBJ_XALLOC_NO_FLUSH);
	EOF

	cat >"$outputFolderPath"/constants/POBJ_XADD.rs <<-EOF
		pub const POBJ_XADD_NO_FLUSH: u64 = POBJ_FLAG_NO_FLUSH;
		pub const POBJ_XADD_VALID_FLAGS: u64 = POBJ_XADD_NO_FLUSH;
	EOF

	cat >"$outputFolderPath"/constants/PMEMOBJ_MAX.rs <<-EOF
		pub const PMEMOBJ_MAX_ALLOC_SIZE: size_t = 0x3FFDFFFC0;
	EOF

	cat >>"$outputFolderPath"/constants.rs <<-EOF
		include!("bindgen/constants/POBJ_FLAG.rs");
		include!("bindgen/constants/POBJ_XALLOC.rs");
		include!("bindgen/constants/POBJ_XADD.rs");
		include!("bindgen/constants/PMEMOBJ_MAX.rs");
	EOF
}
