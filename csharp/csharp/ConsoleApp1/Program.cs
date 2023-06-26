using System.Runtime.CompilerServices;

using Research.RayTracer.Draw;

namespace Research {
	internal class Program {
		private static void OutputInt(uint val) {
			Console.WriteLine("Integer: 0x{0:X8}", val);
			var b = BitConverter.GetBytes(val);
			Console.Write("Bytes  : ");
			for (int i = b.Length - 1; i >= 0; i--) {
				Console.Write("0x{0:X2},", b[i]);
			}
			Console.WriteLine();
		}

		private static void SetBytes(byte r, byte g, byte b) {
			uint x = (uint)(b << 24 | g << 16 | r << 8 | 255);
			OutputInt(x);
		}

		static void Main() {
			Canvas c = new(200, 100);
			c.Clear(new Color(255, 0, 0));
			c.WriteFile("out.png");
		}
	}
}