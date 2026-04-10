package com.example.myapplication4

import android.content.Context
import android.graphics.Color
import android.graphics.drawable.Drawable
import android.graphics.drawable.GradientDrawable
import android.util.AttributeSet
import android.view.View
import com.google.android.material.textfield.TextInputEditText

class SquareView @JvmOverloads constructor(
    context: Context,
    attrs: AttributeSet? = null,
    defStyle: Int = 0

): View(context, attrs, defStyle)  {
    var color = -1
    init {

    }

    private fun init(attrs: AttributeSet?, defStyle: Int) {

        val attributes =
            context.obtainStyledAttributes(
                attrs, R.styleable.CustomInputView,
                defStyle, 0
            )
        color = attributes.getColor(R.styleable.CustomInputView_customColor, -1)
    }
    override fun setBackground(background: Drawable?) {
        if (background!= null && color != -1){
           var a = background as GradientDrawable
            a.setColor(color)

        }
        super.setBackground(background)
    }
}