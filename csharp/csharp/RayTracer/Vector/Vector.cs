using System.Collections;
using System.Numerics;

namespace Research.RayTracer.Vector {
	public class Vector<T> : IEnumerable<T>, IEquatable<Vector<T>> where T : INumber<T> {
		private readonly List<T> _values;

		public Vector() {
			_values = new List<T>();
		}
		public Vector(int size) : this() {
			_values.AddRange(new T[size]);
		}
		public Vector(IEnumerable<T> values) : this() {
			_values.AddRange(values);
		}

		public int Count { get { return _values.Count; } }

		public T this[int idx] {
			get {
				if (idx < 0 || idx >= _values.Count) throw new ArgumentOutOfRangeException(nameof(idx));
				return _values[idx];
			}
			set {
				if (idx < 0 || idx >= _values.Count) throw new ArgumentOutOfRangeException(nameof(idx));
				_values[idx] = value;
			}
		}

		public void Add(T item) { _values.Add(item); }

		public T Dot(Vector<T> other) {
			var retval = _values[0] * other._values[0];
			for(int i = 1; i < Count; i++) {
				retval += _values[i] * other._values[i];
			}
			return retval;
		}

		public IEnumerator<T> GetEnumerator() {
			return _values.GetEnumerator();
		}

		IEnumerator IEnumerable.GetEnumerator() {
			return this.GetEnumerator();
		}

		public bool Equals(Vector<T>? other) {
			if (other == null) return false;
			if (other.Count != _values.Count) return false;
			for(int i = 0; i < _values.Count; i++) {
				if (_values[i] != other[i]) return false;
			}
			return true;
		}

		public override bool Equals(object? obj) {
			return Equals(obj as Vector<T>);
		}

		public override int GetHashCode() {
			return _values.Aggregate(0, (a, b) => a.GetHashCode() ^ b.GetHashCode());
		}
	}
}
