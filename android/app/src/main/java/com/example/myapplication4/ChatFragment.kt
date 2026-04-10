package com.example.myapplication4

import android.os.Bundle
import androidx.fragment.app.Fragment
import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import android.view.ViewGroup.LayoutParams
import android.view.ViewGroup.MarginLayoutParams
import android.widget.LinearLayout
import android.widget.TextView
import androidx.constraintlayout.widget.ConstraintLayout
import androidx.constraintlayout.widget.ConstraintSet
import com.example.myapplication4.databinding.ActivityChatBinding
import com.example.myapplication4.databinding.SnapChatBinding
import com.example.myapplication4.databinding.SnapChatTemplateBinding


// TODO: Rename parameter arguments, choose names that match
// the fragment initialization parameters, e.g. ARG_ITEM_NUMBER
private const val ARG_PARAM1 = "param1"
private const val ARG_PARAM2 = "param2"

/**
 * A simple [Fragment] subclass.
 * Use the [ChatFragment.newInstance] factory method to
 * create an instance of this fragment.
 */
class ChatFragment : Fragment() {
    // TODO: Rename and change types of parameters
    private var param1: String? = null
    private var param2: String? = null

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        arguments?.let {
            param1 = it.getString(ARG_PARAM1)
            param2 = it.getString(ARG_PARAM2)
        }
    }

    override fun onCreateView(
        inflater: LayoutInflater, container: ViewGroup?,
        savedInstanceState: Bundle?
    ): View? {
        // Inflate the layout for this fragment
        //val mainBindingg = ActivityChatBinding.inflate(layoutInflater)
       // val layout_main = mainBindingg.root
        val binding = SnapChatTemplateBinding.inflate(layoutInflater)
        val binding2 = SnapChatTemplateBinding.inflate(layoutInflater)
        val view2 = binding2.root
        view2.id = View.generateViewId()
        val lin_layout = LinearLayout(this.context)
        lin_layout.orientation = LinearLayout.VERTICAL


        val layoutParam = LayoutParams(LayoutParams.WRAP_CONTENT, LayoutParams.WRAP_CONTENT)

        binding.textMessage.text = "hello nick! its a good day isnt it? <3\n\n\n asdfjaopfsdj"
        val view : ConstraintLayout = binding.root
        val set: ConstraintSet = ConstraintSet()
       // val binding2 = SnapChatTemplateBinding.inflate(layoutInflater.cloneInContext(view.context ))
      //  val text =  TextView(view.context)
        //text.id = View.generateViewId()
        //text.text ="HAKUNA MATATATAAA e lindo dizerrr parte 2:D\n asdasda asdjfhasifdhfuhsdjas"
       // view.addView(text)
        //view.addView(view2)
        lin_layout.addView(view)
        lin_layout.addView(view2)


        //text.layoutParams=layoutParam
       // text.layoutParams = layoutParam
       // MarginLayoutParams()

       //set.clone(view)
      // set.connect(text.id, ConstraintSet.TOP, binding.imageView.id, ConstraintSet.BOTTOM, 25)
      //  set.connect(binding2.imageView.id, ConstraintSet.TOP, binding.imageView.id, ConstraintSet.BOTTOM)
      //  set.applyTo(view)



        //view.addView(binding2.root)
        //val view2 = getView( R.layout.snap_chat_template)
       // val view3 = inflater.inflate(R.layout.fragment_chat, null)

        //view.addView(view2)

        return inflater.inflate(R.layout.fragment_chat, lin_layout, true)
        //return layout_main
    }

    override fun onViewCreated(view: View, savedInstanceState: Bundle?) {

        super.onViewCreated(view, savedInstanceState)
    }

    companion object {
        /**
         * Use this factory method to create a new instance of
         * this fragment using the provided parameters.
         *
         * @param param1 Parameter 1.
         * @param param2 Parameter 2.
         * @return A new instance of fragment ChatFragment.
         */
        // TODO: Rename and change types and number of parameters
        @JvmStatic
        fun newInstance(param1: String, param2: String) =
            ChatFragment().apply {
                arguments = Bundle().apply {
                    putString(ARG_PARAM1, param1)
                    putString(ARG_PARAM2, param2)
                }
            }
    }
}