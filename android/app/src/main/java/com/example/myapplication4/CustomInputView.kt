package com.example.myapplication4

import android.content.Context
import android.content.res.Resources
import android.graphics.Bitmap
import android.graphics.Canvas
import android.graphics.Color
import android.graphics.Paint
import android.graphics.Rect
import android.graphics.RectF
import android.graphics.Typeface
import android.util.AttributeSet
import android.util.DisplayMetrics
import android.util.TypedValue
import android.view.MotionEvent
import android.view.ViewGroup
import android.view.WindowManager
import android.view.WindowMetrics
import android.widget.EditText
import android.widget.ImageView.ScaleType.*
import android.widget.TextView
import androidx.appcompat.widget.AppCompatEditText
import androidx.appcompat.widget.AppCompatTextView
import com.google.android.material.textfield.TextInputEditText


class CustomInputView @JvmOverloads constructor(
    context: Context,
    attrs: AttributeSet? = null,
    defStyle: Int = 0

): AppCompatTextView (context, attrs, defStyle) {
    var custom_color : Int = Color.GREEN
    var size : Float = 50.0F
    var textPaint = Paint(Paint.ANTI_ALIAS_FLAG)
    var potato = 10
    public enum class Linebreak {SingleChar, WrapWord}
    var linebreak: Linebreak = Linebreak.WrapWord
    val outer_padding= Vec2(10.0F ,5.0F)
    val inner_padding = Vec2(0.0F, 10.0F)
    var limit_x = get_limit_x()
    var initial_x = outer_padding.x + inner_padding.x
    var initial_y = outer_padding.y + inner_padding.y
    var bubble_color: Int = 255
    var bubble_stroke_color: Int = 0
    var bubble_stroke_width = 8.0F
    lateinit var textBitMap: Bitmap
    lateinit var textCanvas: Canvas
    var bounds = Vec2(100.0F, 100.0F)
    var background_enabled = true
    var allowItalic = true
    init {
        isFocusable = true
        isClickable = true
        isFocusableInTouchMode = true
        //super.setText("ALDJALDKJAKLSDJ")
        init(attrs, defStyle)
    }
    fun get_limit_x(): Float{
        return (measuredWidth - inner_padding.x)
    }

    //override fun setText(text: CharSequence?, type: BufferType?) {
        //"super.setText(text, type)"
        //this.setText("AAAAAAAAA")
        //super.setText("i hate you", type)
    //}
    fun get_default_text_paint(): Paint{
        return(Paint(Paint.ANTI_ALIAS_FLAG).apply {
            color = currentTextColor
            textSize = size
        })
    }

    fun get_special_text_paint(): Paint{
        return(Paint(Paint.ANTI_ALIAS_FLAG).apply {
            color = custom_color
            textSize = size
            if (allowItalic)
            typeface = Typeface.create(Typeface.DEFAULT, Typeface.ITALIC)
            else
                typeface = Typeface.create(Typeface.DEFAULT, Typeface.BOLD)
        })
    }

    private fun init(attrs: AttributeSet?, defStyle: Int) {

        val attributes =
            context.obtainStyledAttributes(attrs, R.styleable.CustomInputView,
                        defStyle, 0)
        custom_color = attributes.getColor(R.styleable.CustomInputView_customColor, Color.RED )
        size = this.textSize
        var t = attributes.getString(R.styleable.CustomInputView_text)
        if (t!=null){
            this.text = text
        }
        var a = attributes.getInt(R.styleable.CustomInputView_lineBreak, 1)
        linebreak = Linebreak.values()[a]
        bubble_color = attributes.getColor(R.styleable.CustomInputView_bubbleColor, Color.parseColor("#E7E7E7"))
        bubble_stroke_color = attributes.getColor(R.styleable.CustomInputView_bubbleStrokeColor, Color.parseColor("#ADC7FF"))
        bubble_stroke_width = attributes.getFloat(R.styleable.CustomInputView_bubbleStrokeWidth, 8.0F)
        limit_x = get_limit_x()
        background_enabled = attributes.getBoolean(R.styleable.CustomInputView_backgroundEnabled, true)
        allowItalic = attributes.getBoolean(R.styleable.CustomInputView_allowItalic, true)
    //  LayoutInflater.from(context).inflate(R.layout.custom_input_view, this, true)
        //this.background = R.drawable.round_bubble
        //  this.setText("OMG PLEASE GOD HELP")
        //this.setText("i like bob")

      textPaint = get_default_text_paint()
    attributes.recycle()


    }


   fun pixelsToSp(context: Context, px: Float): Float {
       val scaledDensity = context.resources.displayMetrics.scaledDensity
       return px / scaledDensity
   }
    // private var textPaint = Paint(Paint.ANTI_ALIAS_FLAG).apply {
       // color = Color.CYAN
      //  textSize = TypedValue.applyDimension(TypedValue.COMPLEX_UNIT_SP, textSize, context.resources.displayMetrics)
       // textSize = size
    //typeface = Typeface.create(Typeface.DEFAULT, Typeface.ITALIC)
  //  }

    fun convertPixelsToDp(context: Context, px: Float): Float {
        val displayMetrics = context.resources.displayMetrics
        return TypedValue.applyDimension(TypedValue.COMPLEX_UNIT_PX, px, displayMetrics)
    }

    fun update_paint(){
        textPaint.color = custom_color
        textPaint.textSize = this.textSize
    }

   override fun onDraw(canvas: Canvas) {
       // super.onDraw(canvas)
       limit_x = get_limit_x()
       if (background_enabled)
            draw_background(canvas)
       canvas.drawBitmap(textBitMap,0.0F,0.0F,null)
   //render_text(canvas)
        //canvas.drawText(this.text.toString(), 0.0F, size, textPaint)
    }

    private fun draw_background(canvas : Canvas){
        var rectPaint = Paint(Paint.ANTI_ALIAS_FLAG).apply {
            style = Paint.Style.STROKE
            color = bubble_stroke_color
            strokeWidth = bubble_stroke_width
        }
        var rectFill =  Paint().apply {
            style = Paint.Style.FILL
            color = bubble_color}
        var rect = RectF(  inner_padding.x, inner_padding.y,
                bounds.x, bounds.y + inner_padding.y)
       // canvas.drawDoubleRoundRect()
        canvas.drawRoundRect(rect, 100.0F, 100.0F, rectFill)
        canvas.drawRoundRect(rect, 100.0F, 100.0F, rectPaint)
       // canvas.rect
       // canvas.drawRect
    }

    fun render_text(canvas: Canvas){
        var max_x = 0.0F
        var offset_x = 0.0F
        var offset_y = inner_padding.y
        val length = this.text.toString().length
        val text = this.text.toString()
        val size = this.textSize
        offset_y += size
        var first_letter = false
        var wrap_word = false
        var matches = mutableListOf<SpecialMatch>()
        var ignore_prev_chars = 0

        for (i in  0 until length){
            var char = text[i]
            if (char == '\n'){
                if (isSingleLine) continue
                if (offset_x> max_x) max_x= offset_x
                offset_y+=size
                offset_x = 0.0F
                continue
            }
            if (char == '*'){
                if (i==0 || text[i-1]!='\\'){
                    val index = find_matching(text, i, '*')
                    if (index != -1){
                        var match = SpecialMatch(index, '*', this)
                        matches.add(match)
                        match.start_match()
                        ignore_prev_chars = 1
                       // textPaint.color= custom_color
                       // textPaint.typeface = Typeface.create(Typeface.DEFAULT, Typeface.ITALIC)
                        continue
                    }
                }
            }
            var break_loop = false
            for (match in 0 until matches.count()){
                var case = matches[match]
                if (case.is_match(char)){
                    case.finish_match()
                    matches.removeAt(match)
                    break_loop = true
                    break
                }
            }
            if (break_loop) continue
            if (linebreak == Linebreak.WrapWord && !isSingleLine){
                first_letter = is_first_letter(text,i, ignore_prev_chars)
                ignore_prev_chars = 0
                if(first_letter){
                    //var total = offset_x +  textPaint.measureText(char.toString())
                    var total = word_length_bytes(text, i, limit_x)
                    if (total<= limit_x - offset_x ){
                        wrap_word = true
                    }
                    else if(total<= limit_x){
                        wrap_word = true
                        if (offset_x> max_x) max_x= offset_x
                        offset_y += size
                        offset_x = 0.0F;
                    }
                    else{
                        wrap_word = false
                    }

                    first_letter = false
                }
            }
            if (isSingleLine){
                val char = text[i].toString()
                val charWidth = textPaint.measureText(char.toString())
                if (offset_x + charWidth + initial_x > limit_x){
                    break
                }
                canvas.drawText(char.toString(),offset_x + initial_x, offset_y, textPaint)
                offset_x+=charWidth
            }
            else if (wrap_word == false || linebreak == Linebreak.SingleChar) {
                var char = text[i].toString()
                val charWidth = textPaint.measureText(char.toString())
                if (offset_x + charWidth + initial_x > limit_x && offset_x != 0.0F ){
                    if (offset_x> max_x) max_x= offset_x
                    offset_y += size
                    offset_x = 0.0F
                }
                canvas.drawText(char.toString(),offset_x + initial_x, offset_y, textPaint)
                offset_x+=charWidth
            }
            else if (wrap_word){
                val charWidth = textPaint.measureText(char.toString())
                if(char == ' '){
                    wrap_word = false
                    if (offset_x + charWidth + initial_x > limit_x){
                        if (offset_x> max_x) max_x= offset_x
                        offset_x = 0.0F
                        offset_y += size
                        continue
                    }
                }
                canvas.drawText(char.toString(),offset_x + initial_x, offset_y, textPaint)
                offset_x+=charWidth
            }
        }
        if (offset_x> max_x) max_x= offset_x
        bounds.y= offset_y
        bounds.x = max_x
    }

    private fun is_first_letter(text: String, i: Int, ignore_prev_chars:Int): Boolean{
        if(i != 0){
            if (is_space_char(text[i - 1 - ignore_prev_chars]) &&  !is_space_char(text[i])) {
                return true
            }
        }else if (text[i]!=' ') {
            return  true
        }
        return false
    }
    private fun is_space_char(char: Char) : Boolean{
        if (char == ' ' || char == '\n'){
            return true
        }
        return false
    }
    private fun word_length_bytes(text: String, i: Int, limit: Float): Float{
        var total = 0.0F
        for (j in i until text.length) {
            var char = text[j]
            if (is_space_char(char)){
                return total
            }
            val charWidth = textPaint.measureText(char.toString())
            total+= charWidth 
            if (total>= limit){
                return total
            }
        }
        return total
    }

    private fun find_matching(text:String, i:Int, find_char:Char): Int{
        for (j in i+1 until text.length){
            var char = text[j]
            if (char== find_char && text[j-1]!='\\'){
                return j
            }
        }
        return -1
    }

    fun redraw(){
        val display_metrics = Resources.getSystem().displayMetrics
        val height = display_metrics.heightPixels
        val width = display_metrics.widthPixels
        textBitMap =  Bitmap.createBitmap(width,height, Bitmap.Config.ARGB_8888)
        textCanvas = Canvas(textBitMap)
        this.render_text(textCanvas)
        invalidate()
    }

    override fun onFocusChanged(focused: Boolean, direction: Int, previouslyFocusedRect: Rect?) {
        super.onFocusChanged(focused, direction, previouslyFocusedRect)
    }

    override fun onTouchEvent(event: MotionEvent?): Boolean {
        requestFocus()
        return true
        return super.onTouchEvent(event)
    }
    override fun onMeasure(widthMeasureSpec: Int, heightMeasureSpec: Int) {
        //setMeasuredDimension(1000,1000)
     //   val displayMetrics = DisplayMetrics()
     //   val windowManager = context.getSystemService(Context.WINDOW_SERVICE) as WindowManager
     //   windowManager.getDefaultDisplay().getMetrics(displayMetrics)
       // var display = context.display
        val display_metrics = Resources.getSystem().displayMetrics

        val height = display_metrics.heightPixels
        val width = display_metrics.widthPixels
        textBitMap =  Bitmap.createBitmap(width,height, Bitmap.Config.ARGB_8888)
        textCanvas = Canvas(textBitMap)
        var lp = layoutParams

        limit_x = width - initial_x - outer_padding.x - inner_padding.x
        if (lp.width != ViewGroup.LayoutParams.WRAP_CONTENT){
            var pixel_x = MeasureSpec.getSize(widthMeasureSpec)
            limit_x = pixel_x.toFloat()  - initial_x - outer_padding.x
        }
       // redraw()
        this.render_text(textCanvas)

        var x = bounds.x + initial_x
        var y = bounds.y + inner_padding.y

        if (lp.width != ViewGroup.LayoutParams.WRAP_CONTENT){
            var pixel_x = MeasureSpec.getSize(widthMeasureSpec)
            x = pixel_x.toFloat()
            bounds.x = pixel_x - outer_padding.x - inner_padding.x
        }
        if (lp.height != ViewGroup.LayoutParams.WRAP_CONTENT&& !isSingleLine){
            var pixel_y = MeasureSpec.getSize(heightMeasureSpec)
            y = pixel_y.toFloat()
            bounds.y = y - outer_padding.y - inner_padding.y

        }


        setMeasuredDimension(x.toInt(), y.toInt() )
    //super.onMeasure(bounds.x.toInt(), bounds.y.toInt())
    }
    class SpecialMatch(val index: Int, val char: Char, var view : CustomInputView){
        public fun start_match(){
            if (char == '*'){
                this.view.textPaint = view.get_special_text_paint()
            }
        }
        public fun finish_match(){
            if (char == '*'){
                this.view.textPaint = view.get_default_text_paint()
            }
        }

        public  fun is_match(char2: Char): Boolean{
            if (char2 == char){
                return true
            }
            return false
        }

    }

    class Vec2(var x: Float, var y: Float){

       // var x : Float = x
       // var y : Float = y
        operator fun plus(other: Vec2) : Vec2{
            //return Vec2(10.0F , 5.0F)
            return Vec2(x + other.x, y + other.y)
        }
    }


}