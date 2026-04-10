package com.example.myapplication4

import android.content.Context
import android.util.AttributeSet
import androidx.appcompat.widget.AppCompatTextView
import com.google.android.material.textfield.TextInputEditText

class TextInputWithBackground @JvmOverloads constructor(
    context: Context,
    attrs: AttributeSet? = null,
    defStyle: Int = 0

): TextInputEditText(context, attrs, defStyle) {

    init{
        isFocusable = true
        isClickable = true
        isFocusableInTouchMode = true
    }
    override fun onMeasure(widthMeasureSpec: Int, heightMeasureSpec: Int) {
        super.onMeasure(widthMeasureSpec, heightMeasureSpec)


    }

    override fun onLayout(changed: Boolean, left: Int, top: Int, right: Int, bottom: Int) {
        super.onLayout(changed, left, top, right, bottom)
     //   setLeft(0)
       // setTop(0)
        // Set the position of the view
        // Set your desired Y position


        // Call requestLayout() to ensure a layout pass with the new position
     //   requestLayout()
    }
}