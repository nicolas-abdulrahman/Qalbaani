package com.example.myapplication4

import android.content.Context
import android.graphics.Bitmap
import android.graphics.Canvas
import android.graphics.Color
import android.graphics.Paint
import android.graphics.RectF
import android.graphics.drawable.BitmapDrawable
import android.util.AttributeSet
import android.view.View
import android.view.ViewGroup
import android.widget.RelativeLayout
import androidx.core.view.marginTop
import com.google.android.material.textfield.TextInputEditText

class CustomBackground @JvmOverloads constructor(
    context: Context,
    attrs: AttributeSet? = null,
    defStyle: Int = 0

): RelativeLayout(context, attrs, defStyle) {
    val outer_padding= CustomInputView.Vec2(10.0F, 5.0F)
    val inner_padding = CustomInputView.Vec2(20.0F, 10.0F)
    var bubble_color: Int = 255
    var bubble_stroke_color: Int = 0
    var bubble_stroke_width = 8.0F
    var bounds = CustomInputView.Vec2(100.0F, 100.0F)

    init {
        init(attrs, defStyle)
    }
    private fun init(attrs: AttributeSet?, defStyle: Int) {
        val attributes =
            context.obtainStyledAttributes(attrs, R.styleable.CustomInputView,
                defStyle, 0)
        bubble_color = attributes.getColor(R.styleable.CustomInputView_bubbleColor, Color.parseColor("#E7E7E7"))
        bubble_stroke_color = attributes.getColor(R.styleable.CustomInputView_bubbleStrokeColor, Color.parseColor("#ADC7FF"))
        bubble_stroke_width = attributes.getFloat(R.styleable.CustomInputView_bubbleStrokeWidth, 8.0F)
    }

    override fun forceLayout() {
        layout(0,1000,200,200)
        requestLayout()
    }

    override fun onLayout(changed: Boolean, l: Int, t: Int, r: Int, b: Int) {
        super.onLayout(changed, l, 0, r, b)
       // print("l: $l t:$t r:$r b:$b")
        var leftOffset = 10 // Initial left offset

        for (i in 0 until childCount) {
            val childView: View = getChildAt(i)

            // Calculate and set the child's position with the left offset
            childView.layout(leftOffset, 0, childView.measuredWidth, childView.measuredHeight)

            // Increase the left offset for the next view
            //leftOffset += childView.measuredWidth + 20 // Add margin between views
        }

        requestLayout()

        // background = BitmapDrawable()
    }

    override fun dispatchDraw(canvas: Canvas) {
        draw_background(canvas)
        super.dispatchDraw(canvas)

    }
    override fun onDraw(canvas:Canvas){

    }
    private fun draw_background(canvas : Canvas){
        bounds.x = measuredWidth.toFloat()
        bounds.y = measuredHeight.toFloat()
        var rectPaint = Paint(Paint.ANTI_ALIAS_FLAG).apply {
            style = Paint.Style.STROKE
            color = bubble_stroke_color
            strokeWidth = bubble_stroke_width
        }
        var rectFill =  Paint().apply {
            style = Paint.Style.FILL
            color = bubble_color}
        var rect = RectF( 0.0F, 0.0F,
            measuredWidth.toFloat() , measuredHeight.toFloat())
        // canvas.drawDoubleRoundRect()
        canvas.drawRoundRect(rect, 100.0F, 100.0F, rectFill)
        canvas.drawRoundRect(rect, 100.0F, 100.0F, rectPaint)
        // canvas.rect
        // canvas.drawRect
    }

    override fun onMeasure(widthMeasureSpec: Int, heightMeasureSpec: Int) {
        super.onMeasure(widthMeasureSpec, heightMeasureSpec)
      //  setMeasuredDimension(measuredWidth,
        //    measuredHeight)
    }
}