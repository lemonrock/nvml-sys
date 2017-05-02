# This file is part of nvml-sys. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml-sys/master/COPYRIGHT. No part of nvml-sys, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
# Copyright © 2016 The developers of nvml-sys. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/nvml-sys/master/COPYRIGHT.


bindingsName='nvml'
rootIncludeFileName='nvml.h'
macosXHomebrewPackageNames=''
alpineLinuxPackageNames='rsync make gcc linux-headers libunwind-dev linux-grsec-dev'
clangAdditionalArguments=''
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
}
