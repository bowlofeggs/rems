# Copyright © 2020 Randy Barlow
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, version 3 of the License.
# 
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.
# 
# You should have received a copy of the GNU General Public License
# along with this program.  If not, see <http://www.gnu.org/licenses/>.
"""Generate a signal for use with rems' 1D simulator."""

import math
import os

import bson


signal = [math.exp(-0.5 * (((2048.0 - t)/512.0)**2.0)) * math.sin(t/100.) for t in range(4092)]
with open(os.path.join(os.path.dirname(__file__), '1d_signal.bson'), 'bw') as signal_f:
    signal_f.write(bson.BSON.encode({'ex': signal, '_version': 0}))
