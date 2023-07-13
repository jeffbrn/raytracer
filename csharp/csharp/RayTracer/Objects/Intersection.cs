namespace Research.RayTracer.Objects {
	public class Intersection : IEquatable<Intersection> {
		public Intersection(double t, Sphere obj) {
			T = t;
			Object = obj;
		}

		public double T { get; private set; }

		public Sphere Object { get; private set; }

		public bool Equals(Intersection? other) {
			if (other == null) return false;
			return T == other.T && Object == other.Object;
		}

		public override bool Equals(object? obj) {
			return Equals(obj as Intersection);
		}

		public override int GetHashCode() {
			return T.GetHashCode() ^ Object.GetHashCode();
		}
	}
}
