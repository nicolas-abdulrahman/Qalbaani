package com.example.myapplication4

import android.content.Context
import android.util.AttributeSet
import android.view.MotionEvent
import android.view.View
import android.widget.ScrollView
import com.google.android.material.textfield.TextInputEditText

class LockableScrollView  @JvmOverloads constructor(
    context: Context,
    attrs: AttributeSet? = null,
    defStyle: Int = 0

): ScrollView (context, attrs, defStyle) {
    var mScrollable = true
    override fun onTouchEvent(ev: MotionEvent?): Boolean {
        if (mScrollable== false) {
           return false
        }
        return super.onTouchEvent(ev)
    }

    override fun onInterceptTouchEvent(ev: MotionEvent?): Boolean {
       // return super.onInterceptTouchEvent(ev)
        return false
    }
}