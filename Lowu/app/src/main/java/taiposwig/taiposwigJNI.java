/* ----------------------------------------------------------------------------
 * This file was automatically generated by SWIG (http://www.swig.org).
 * Version 4.0.1
 *
 * Do not make changes to this file unless you know what you are doing--modify
 * the SWIG interface file instead.
 * ----------------------------------------------------------------------------- */

package taiposwig;

public class taiposwigJNI {
  public final static native void CResponseItem_key_set(long jarg1, CResponseItem jarg1_, String jarg2);
  public final static native String CResponseItem_key_get(long jarg1, CResponseItem jarg1_);
  public final static native void CResponseItem_value_set(long jarg1, CResponseItem jarg1_, String jarg2);
  public final static native String CResponseItem_value_get(long jarg1, CResponseItem jarg1_);
  public final static native long new_CResponseItem();
  public final static native void delete_CResponseItem(long jarg1);
  public final static native void delete_data(long jarg1);
  public final static native void execute(long jarg1, String jarg2);
  public final static native void handle_event(long jarg1, int jarg2);
  public final static native String initial_html(long jarg1);
  public final static native boolean is_shutdown_required(long jarg1);
  public final static native long make_data(String jarg1);
  public final static native String response_error(long jarg1);
  public final static native long response_item(long jarg1, int jarg2);
  public final static native int response_num_items(long jarg1);
  public final static native boolean response_ok(long jarg1);
}
