using Research.RayTracer.Vector;

namespace Research.RayTracer.Objects {
	public class Sphere: IEquatable<Sphere> {
		public Sphere() {
			Center = RayTuple<double>.Origin();
			Radius = 1.0;
		}

		public RayTuple<double> Center { get; private set; }

		public double Radius { get; private set; }

		public Tuple<double, double>? Intersect(Ray<double> ray) {
			var sphere_to_ray = ray.Origin - Center;
			var a = ray.Direction.DotProd(ray.Direction);
			var b = 2 *ray.Direction.DotProd(sphere_to_ray);
			var c = sphere_to_ray.DotProd(sphere_to_ray) - 1;
			var discriminant = b * b - 4 * a * c;
			if (discriminant < 0) return null;
			var t1 = (-b - Math.Sqrt(discriminant)) / (2 * a);
			var t2 = (-b + Math.Sqrt(discriminant)) / (2 * a);
			return Tuple.Create(t1, t2);
		}

		public bool Equals(Sphere? other) {
			if (other == null) return false;
			return Center.Equals(other.Center) && Radius == other.Radius;
		}

		public override bool Equals(object? obj) {
			return Equals(obj as Sphere);
		}

		public override int GetHashCode() {
			return Center.GetHashCode() ^ Radius.GetHashCode();
		}

	}
}
