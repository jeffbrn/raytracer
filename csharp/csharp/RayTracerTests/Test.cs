using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Research.RayTracer.Tests {
	public class Test {
		[Fact]
		public void Check1() {
			uint a = 256;
			var b = BitConverter.GetBytes(a);
			for(int i = 0; i < b.Length; i++) {
				Console.WriteLine("{0},", b[i]);
			}
		}
	}
}
