using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Data;
using System.Drawing;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Windows.Forms;

namespace FirstWinFormApplication
{
    public partial class Form1 : Form
    {
        public Form1()
        {
            InitializeComponent();
        }

        private void button1_Click(object sender, EventArgs e)
        {
            MessageBox.Show("Hello, This is my first C# program :-)");
        }

        private void button2_Click(object sender, EventArgs e)
        {
            this.textBox1.Clear();
        }

        private void Form1_MouseMove(object sender, MouseEventArgs e)
        {
            this.label1.Text = e.X.ToString();
            this.label2.Text = e.Y.ToString();
        }

        private void textBox1_TextChanged(object sender, EventArgs e)
        {
            this.textBox2.Text = this.textBox1.Text;
        }

        private void timer1_Tick(object sender, EventArgs e)
        {
            this.Text = DateTime.Now.ToString();

            Random random = new Random();
            this.BackColor = Color.FromArgb(random.Next(255), random.Next(255), random.Next(255));
        }
    }
}
