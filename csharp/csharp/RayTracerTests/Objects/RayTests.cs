using Research.RayTracer.Objects;
using Research.RayTracer.Vector;

namespace Research.RayTracer.Tests.Objects
{
    public class RayTests
    {

        [Fact]
        public void PositionTests()
        {
            var r = new Ray<int>(RayTuple<int>.Point(2, 3, 4), RayTuple<int>.Vector(1, 0, 0));
            Assert.Equal(r.Position(0), RayTuple<int>.Point(2, 3, 4));
            Assert.Equal(r.Position(1), RayTuple<int>.Point(3, 3, 4));
            Assert.Equal(r.Position(-1), RayTuple<int>.Point(1, 3, 4));
            var r_ = r.Cast<double>();
            Assert.Equal(r_.Position(2.5), RayTuple<double>.Point(4.5, 3.0, 4.0));
        }

    }
}
