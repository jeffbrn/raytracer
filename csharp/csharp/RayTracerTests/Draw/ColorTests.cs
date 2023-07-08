using Research.RayTracer.Draw;

namespace Research.RayTracer.Tests.Draw {
	public class ColorTests {
		[Fact]
		public void NewColor() {
			var c = new Color(-0.5f, 0.4f, 1.7f);
			Assert.Equal(-0.5f, c.Red);
			Assert.Equal(0.4f, c.Green);
			Assert.Equal(1.7f, c.Blue);
		}

		[Fact]
		public void AddColor() {
			var c1 = new Color(0.9f, 0.6f, 0.75f);
			var c2 = new Color(0.7f, 0.1f, 0.25f);
			Assert.Equal(new Color(1.6f, 0.7f, 1.0f), c1 + c2);
		}

		[Fact]
		public void SubtractColor() {
			var c1 = new Color(0.9f, 0.6f, 0.75f);
			var c2 = new Color(0.7f, 0.1f, 0.25f);
			Assert.Equal(new Color(0.2f, 0.5f, 0.5f), c1 - c2);
		}

		[Fact]
		public void MultiplyColor() {
			var c = new Color(0.2f, 0.3f, 0.4f);
			Assert.Equal(new Color(0.4f, 0.6f, 0.8f), c * 2);
			var c1 = new Color(1.0f, 0.2f, 0.4f);
			var c2 = new Color(0.9f, 1.0f, 0.1f);
			Assert.Equal(new Color(0.9f, 0.2f, 0.04f), c1*c2);
		}

		[Fact]
		public void SetBytes() {
			uint val = 0x4080ffff;
			var clr = Color.SetBytes(val);
			Assert.Equal(0.25, clr.Red, 2);
			Assert.Equal(0.5, clr.Green, 2);
			Assert.Equal(1.0, clr.Blue, 2);
		}

		[Fact]
		public void Normalize() {
			var clr = new Color(10, 20, 40).Normalize();
			Assert.Equal(0.25, clr.Red, 2);
			Assert.Equal(0.5, clr.Green, 2);
			Assert.Equal(1.0, clr.Blue, 2);
		}

		[Fact]
		public void GetBytes() {
			var clr = new Color(1.0f, 0.25f, 0.5f);
			var (r, g, b, a) = clr.GetBytes();
			Assert.Equal(255, r);
			Assert.Equal(64, g);
			Assert.Equal(128, b);
			Assert.Equal(255, a);
		}
	}
}
