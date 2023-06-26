namespace WinForm {
	public partial class MainFrm : Form {
		public MainFrm() {
			InitializeComponent();
		}

		private void MainFrm_Load(object sender, EventArgs e) {
			pictureBox1.BackColor = Color.White;
			pictureBox1.Invalidate();
		}

		private void MainFrm_Paint(object sender, PaintEventArgs e) {
		}

#pragma warning disable IDE1006 // Naming Styles
		private void pictureBox1_Paint(object sender, PaintEventArgs e) {
			var client = e.ClipRectangle;
			client.Inflate(-2, -2);
			client.Offset(-1, -1);
			var g = e.Graphics;
			g.DrawRectangle(Pens.BlueViolet, client.Left, client.Top, client.Width, client.Height);
			g.DrawRectangle(Pens.Black, 10, 10, 1, 1);
			//g.FillRectangle(Brushes.Aqua, 10, 10, 200, 100);
		}
#pragma warning restore IDE1006 // Naming Styles
	}
}