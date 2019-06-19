// Copyright 2017-present Kirill Danshin and Gramework contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//

package gramework

import (
	_ "unsafe" // required to use //go:linkname
)

// Nanotime is monotonic time provider.
func Nanotime() int64 {
	return nanotime()
}

//go:noescape
//go:linkname nanotime runtime.nanotime
func nanotime() int64
