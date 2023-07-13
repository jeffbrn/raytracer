using System.Collections;

namespace Research.RayTracer.Objects {
	public class Intersections : IEnumerable<Intersection> {
		private readonly List<Intersection> _intersections = new();

		public Intersections(params Intersection[] i) {
			_intersections.AddRange(i);
		}
		public Intersections(IEnumerable<Intersection> items) {
			_intersections.AddRange(items);
		}

		public int Count => _intersections.Count;
		public Intersection this[int idx] {
			get {
				if (idx < 0  || idx >= _intersections.Count) throw new ArgumentOutOfRangeException(nameof(idx));
				return _intersections[idx];
			}
		}

		public Intersection? Hit() { return _intersections.OrderBy(x => x.T).FirstOrDefault(x => x.T >= 0); }

		public IEnumerator<Intersection> GetEnumerator() {
			return _intersections.GetEnumerator();
		}

		IEnumerator IEnumerable.GetEnumerator() {
			return this.GetEnumerator();
		}
	}
}
