using System.Numerics;
using Research.RayTracer.Vector;

namespace Research.RayTracer.Objects
{
    public class Ray<T> : IEquatable<Ray<T>> where T : INumber<T>
    {
        public Ray(RayTuple<T> origin, RayTuple<T> direction)
        {
            Origin = origin;
            Direction = direction;
        }

        public RayTuple<T> Origin { get; private set; }

        public RayTuple<T> Direction { get; private set; }

        public Ray<TResult> Cast<TResult>() where TResult : INumber<TResult> => new(Origin.Cast<TResult>(), Direction.Cast<TResult>());

        public RayTuple<T> Position(T r)
        {
            return Origin + Direction * r;
        }

        public bool Equals(Ray<T>? other)
        {
            if (other == null) return false;
            return Origin.Equals(other.Origin) && Direction.Equals(other.Direction);
        }

        public override bool Equals(object? obj)
        {
            return Equals(obj as Ray<T>);
        }

        public override int GetHashCode()
        {
            return Origin.GetHashCode() ^ Direction.GetHashCode();
        }

    }
}
