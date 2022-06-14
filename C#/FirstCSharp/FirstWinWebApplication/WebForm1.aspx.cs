using System;
using System.Collections.Generic;
using System.Linq;
using System.Drawing;
using System.Web;
using System.Web.UI;
using System.Web.UI.WebControls;

namespace FirstWinWebApplication
{
    public partial class WebForm1 : System.Web.UI.Page
    {
        protected void Page_Load(object sender, EventArgs e)
        {

        }

        protected void Button1_Click(object sender, EventArgs e)
        {
            if(Int32.TryParse(this.TextBox1.Text, out int value))
            {
                if(value > 0 && value < 100)
                {
                    this.Label1.Text = "OK";

                    Color c = Color.FromArgb(0, 255, 0);
                    this.TextBox1.BackColor = c;
                    this.Label1.ForeColor = c;
                }
                else
                {
                    this.Label1.Text = "Not OK";

                    Color c = Color.FromArgb(255, 127, 0);
                    this.TextBox1.BackColor = c;
                    this.Label1.ForeColor = c;
                }
            }
            else
            {
                this.Label1.Text = "Error";

                Color c = Color.FromArgb(255, 0, 0);
                this.TextBox1.BackColor = c;
                this.Label1.ForeColor = c;
            }
        }
    }
}