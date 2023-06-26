using Research.RayTracer.Draw;

namespace Research.RayTracer.Tests.Draw {
	public class CanvasTests {
		[Fact]
		public void Clear() {
			Canvas c = new (10,5);
			for(int i = 0; i < 5; i++) {
				for(int j = 0; j < 10; j++) {
					var (r, g, b, a) = c[i, j].GetBytes();
					Assert.Equal(0, r);
					Assert.Equal(0, g);
					Assert.Equal(0, b);
					Assert.Equal(255, a);
				}
			}
		}

		[Fact]
		public void SetBackground() {
			Canvas c = new(10, 5);
			Color clr = new(0.25f, 0.5f, 0.751f);
			c.Clear(clr);
			for (int i = 0; i < 5; i++) {
				for (int j = 0; j < 10; j++) {
					var (r, g, b, a) = c[i, j].GetBytes();
					Assert.Equal(64, r);
					Assert.Equal(128, g);
					Assert.Equal(192, b);
					Assert.Equal(255, a);
				}
			}
		}

		[Fact]
		public void SetPixel() {
			Canvas c = new(10, 5);
			Color clr1 = new(0.25f, 0.5f, 0.751f);
			Color clr2 = new(0.751f, 0.5f, 0.25f);
			c[0, 0] = clr1;
			c[9, 4] = clr2;
			var (r, g, b, a) = c[0, 0].GetBytes();
			Assert.Equal(64, r);
			Assert.Equal(128, g);
			Assert.Equal(192, b);
			Assert.Equal(255, a);
			(r, g, b, a) = c[4, 9].GetBytes();
			Assert.Equal(192, r);
			Assert.Equal(128, g);
			Assert.Equal(64, b);
			Assert.Equal(255, a);
		}
	}
}
